use std::collections::HashMap;
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

const EMBEDDED_BCL: &[u8] = include_bytes!("../../../build/bcl");
const EMBEDDED_RT: &[u8] = include_bytes!("../../target/release/libbangla_ffi.a");
const EMBEDDED_RAYLIB: &[u8] = include_bytes!("../../../raylib-6.2_linux_amd64/lib/libraylib.a");

// WebAssembly target (`--wasm`): raylib's PLATFORM_WEB build plus a C copy of
// the runtime in src/lib.rs (compiled by emcc, so no Rust wasm toolchain needed).
const EMBEDDED_RAYLIB_WEB: &[u8] =
    include_bytes!("../../../raylib-6.2_webassembly/lib/libraylib.web.a");
const WASM_RT_C: &str = include_str!("../../runtime/bangla_rt.c");

struct FfiEntry {
    bangla_name: &'static str,
    c_symbol: &'static str,
    arity: usize,
}

// Now you can use natural Bangla (or ASCII) names here!
const FFI_TABLE: &[FfiEntry] = &[
    FfiEntry { bangla_name: "যোগকরো", c_symbol: "bl_add", arity: 2 },
    FfiEntry { bangla_name: "তালিকাতৈরি", c_symbol: "bl_list_new", arity: 0 },
    FfiEntry { bangla_name: "তালিকায়যোগ", c_symbol: "bl_list_push", arity: 2 },
    FfiEntry { bangla_name: "তালিকাহতে", c_symbol: "bl_list_get", arity: 2 },
    FfiEntry { bangla_name: "তালিকায়বসাও", c_symbol: "bl_list_set", arity: 3 },
    FfiEntry { bangla_name: "তালিকাদৈর্ঘ্য", c_symbol: "bl_list_len", arity: 1 },
    FfiEntry { bangla_name: "জানালাখোলো", c_symbol: "bl_init_window", arity: 2 },
    FfiEntry { bangla_name: "জানালাবন্ধ", c_symbol: "bl_window_should_close", arity: 0 },
    FfiEntry { bangla_name: "বন্ধকরো", c_symbol: "bl_close_window", arity: 0 },
    FfiEntry { bangla_name: "গতিসেট", c_symbol: "bl_set_target_fps", arity: 1 },
    FfiEntry { bangla_name: "আঁকাশুরু", c_symbol: "bl_begin_drawing", arity: 0 },
    FfiEntry { bangla_name: "আঁকাশেষ", c_symbol: "bl_end_drawing", arity: 0 },
    FfiEntry { bangla_name: "পটভূমিমুছো", c_symbol: "bl_clear_background", arity: 3 },
    FfiEntry { bangla_name: "আয়তআঁকো", c_symbol: "bl_draw_rect", arity: 7 },
    FfiEntry { bangla_name: "সংখ্যাআঁকো", c_symbol: "bl_draw_number", arity: 7 },
    FfiEntry { bangla_name: "বোতামচাপা", c_symbol: "bl_is_key_pressed", arity: 1 },
    FfiEntry { bangla_name: "এলোমেলো", c_symbol: "bl_random", arity: 2 },
];

const KEYWORDS: &[&str] = &[
    "সংখ্যা", "দশমিক", "স্ট্রিং", "বুলিয়ান", "যদি", "নাহলে",
    "যতক্ষণ", "ফাংশন", "ফিরিয়ে", "দাও", "দেখাও", "সত্য",
    "মিথ্যা", "এবং", "অথবা", "না",
];

fn encode_safe_id(mut n: usize) -> String {
    const SAFE: &[char] = &['ক', 'খ', 'গ', 'ঘ', 'ঙ', 'চ', 'ছ', 'জ', 'ঝ', 'ট'];
    let mut out = String::from("ক");
    if n == 0 {
        out.push(SAFE[0]);
        return out;
    }
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(SAFE[n % SAFE.len()]);
        n /= SAFE.len();
    }
    for d in digits.into_iter().rev() {
        out.push(d);
    }
    out
}

fn llvm_escape(name: &str) -> String {
    name.as_bytes().iter().map(|b| format!("\\{:02X}", b)).collect()
}

fn strip_comments(src: &str) -> String {
    let mut out = String::new();
    for line in src.lines() {
        let mut in_str = false;
        let mut chars = line.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '"' {
                in_str = !in_str;
                out.push(c);
            } else if !in_str && c == '/' && chars.peek() == Some(&'/') {
                break;
            } else {
                out.push(c);
            }
        }
        out.push('\n');
    }
    out
}

fn desugar_void_calls(src: &str) -> String {
    let mut out = String::new();
    let mut tmp_counter = 0usize;
    for line in src.lines() {
        let t = line.trim();
        let is_stmt_keyword = t.starts_with("সংখ্যা ")
            || t.starts_with("দশমিক ")
            || t.starts_with("স্ট্রিং ")
            || t.starts_with("বুলিয়ান ")
            || t.starts_with("যদি")
            || t.starts_with("নাহলে")
            || t.starts_with("যতক্ষণ")
            || t.starts_with("ফাংশন ")
            || t.starts_with("ফিরিয়ে ")
            || t.starts_with("দেখাও");

        if !is_stmt_keyword && t.ends_with(");") && !t.contains('=') && t.contains('(') {
            out.push_str(&format!("__void_sink = {t}\n"));
            tmp_counter += 1;
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

fn is_ident_char(c: char) -> bool {
    let is_bn_digit = ('\u{09E6}'..='\u{09EF}').contains(&c);
    if is_bn_digit {
        return false;
    }
    ('\u{0980}'..='\u{09FF}').contains(&c) || c.is_ascii_alphanumeric() || c == '_'
}

fn preprocess_bgl(src: &str) -> (String, HashMap<String, String>) {
    let with_sink = format!("সংখ্যা __void_sink = ০;\n{src}");
    let no_comments = strip_comments(&with_sink);
    let desugared = desugar_void_calls(&no_comments);

    let mut map: HashMap<String, String> = HashMap::new();
    let mut next_id = 0usize;

    // Pre-register FFI names so we know their exact mangled names
    for e in FFI_TABLE {
        map.entry(e.bangla_name.to_string()).or_insert_with(|| {
            let m = encode_safe_id(next_id);
            next_id += 1;
            m
        });
    }

    let mut out = String::new();
    let mut chars = desugared.chars().peekable();
    while let Some(&c) = chars.peek() {
        if c == '"' {
            out.push(chars.next().unwrap());
            while let Some(sc) = chars.next() {
                out.push(sc);
                if sc == '"' {
                    break;
                }
            }
        } else if ('\u{09E6}'..='\u{09EF}').contains(&c) {
            // Leave Bangla numbers untouched
            while let Some(&dc) = chars.peek() {
                if ('\u{09E6}'..='\u{09EF}').contains(&dc) || dc == '.' {
                    out.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
        } else if is_ident_char(c) {
            let mut word = String::new();
            while let Some(&wc) = chars.peek() {
                if is_ident_char(wc) {
                    word.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            if KEYWORDS.contains(&word.as_str()) {
                out.push_str(&word);
            } else {
                let mangled = map.entry(word).or_insert_with(|| {
                    let m = encode_safe_id(next_id);
                    next_id += 1;
                    m
                });
                out.push_str(mangled);
            }
        } else {
            out.push(chars.next().unwrap());
        }
    }

    (out, map)
}

fn generate_stubs(map: &HashMap<String, String>) -> String {
    let mut out = String::new();
    let mut p_counter = 10_000usize;
    for e in FFI_TABLE {
        let mangled_fn = &map[e.bangla_name];
        let params: Vec<String> = (0..e.arity)
            .map(|_| {
                let p = encode_safe_id(p_counter);
                p_counter += 1;
                format!("সংখ্যা {p}")
            })
            .collect();
        out.push_str(&format!(
            "ফাংশন {mangled_fn}({}) {{ ফিরিয়ে দাও ০; }};\n",
            params.join(", ")
        ));
    }
    out
}

fn patch_ir(ir: &str, map: &HashMap<String, String>) -> String {
    let mut cleaned = String::new();
    let mut skipping = false;

    for line in ir.lines() {
        if skipping {
            if line.trim() == "}" {
                skipping = false;
            }
            continue;
        }
        let is_stub = FFI_TABLE.iter().any(|e| {
            let esc = llvm_escape(&map[e.bangla_name]);
            line.starts_with(&format!("define i32 @\"{esc}\""))
        });
        if is_stub {
            skipping = true;
            continue;
        }
        cleaned.push_str(line);
        cleaned.push('\n');
    }

    for e in FFI_TABLE {
        let esc = llvm_escape(&map[e.bangla_name]);
        cleaned = cleaned.replace(&format!("@\"{esc}\""), &format!("@{}", e.c_symbol));
    }

    for e in FFI_TABLE {
        let args = vec!["i32"; e.arity].join(", ");
        cleaned.push_str(&format!("\ndeclare i32 @{}({})\n", e.c_symbol, args));
    }
    cleaned
}

/// Link the patched IR + C runtime + raylib(web) into .html/.js/.wasm with emcc.
fn link_wasm(work_dir: &Path, ll_path: &Path, out: &Path) -> Result<(), String> {
    let rt_c = work_dir.join("bangla_rt.c");
    fs::write(&rt_c, WASM_RT_C).map_err(|e| format!("cannot write runtime: {e}"))?;
    let web_lib = work_dir.join("libraylib.web.a");
    fs::write(&web_lib, EMBEDDED_RAYLIB_WEB).map_err(|e| format!("cannot write raylib: {e}"))?;

    let status = Command::new("emcc")
        .current_dir(work_dir)
        .arg(ll_path)
        .arg(&rt_c)
        .arg(&web_lib)
        .args([
            "-O2",
            "-Wno-override-module",
            "-sUSE_GLFW=3",
            // BangLang programs are a blocking `while` loop; ASYNCIFY lets that
            // loop yield to the browser (raylib's frame wait uses emscripten_sleep).
            "-sASYNCIFY",
            "-sALLOW_MEMORY_GROWTH=1",
            "-sINITIAL_MEMORY=67108864",
            "-sSTACK_SIZE=1048576",
            "-o",
        ])
        .arg(out)
        .status()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                "emcc not found. Install the Emscripten SDK and run `source ./emsdk_env.sh` \
                 in this shell first (https://emscripten.org/docs/getting_started/downloads.html)"
                    .to_string()
            } else {
                format!("failed to run emcc: {e}")
            }
        })?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("emcc failed ({status})"))
    }
}

fn main() {

    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: bangc <input.bgl> [-o <output>] [--wasm]");
        eprintln!("  --wasm   build for the browser with emcc (default output: bangla.html;");
        eprintln!("           also chosen automatically when -o ends in .html, .js or .wasm)");
        std::process::exit(1);
    }

    let mut input_path: Option<PathBuf> = None;
    let mut output_path = PathBuf::from("program");
    let mut output_given = false;
    let mut wasm = false;

    let mut i = 0;
    while i < args.len() {
        if args[i] == "-o" && i + 1 < args.len() {
            i += 1;
            output_path = PathBuf::from(&args[i]);
            output_given = true;
        } else if args[i] == "--wasm" {
            wasm = true;
        } else {
            input_path = Some(PathBuf::from(&args[i]));
        }
        i += 1;
    }

    if !wasm {
        wasm = matches!(
            output_path.extension().and_then(|e| e.to_str()),
            Some("html") | Some("js") | Some("wasm")
        );
    }
    if wasm && !output_given {
        output_path = PathBuf::from("bangla.html");
    }

    let input_path = input_path.expect("Missing <input.bgl>");
    let user_src = fs::read_to_string(&input_path).expect("Could not read input .bgl file");

    let (mangled_src, map) = preprocess_bgl(&user_src);
    let combined = format!("{}\n{}", generate_stubs(&map), mangled_src);

    let work_dir = env::temp_dir().join(format!("bangc_{}", std::process::id()));
    fs::create_dir_all(&work_dir).unwrap();

    let bcl_bin = work_dir.join("bcl");
    fs::write(&bcl_bin, EMBEDDED_BCL).unwrap();
    fs::set_permissions(&bcl_bin, fs::Permissions::from_mode(0o755)).unwrap();

    let rt_lib = work_dir.join("libbangla_ffi.a");
    let raylib_a = work_dir.join("libraylib.a");
    if !wasm {
        fs::write(&rt_lib, EMBEDDED_RT).unwrap();
        fs::write(&raylib_a, EMBEDDED_RAYLIB).unwrap();
    }

    let combined_bgl = work_dir.join("input.bgl");
    fs::write(&combined_bgl, combined).unwrap();

    let status = Command::new(&bcl_bin)
        .arg(&combined_bgl)
        .current_dir(&work_dir)
        .status()
        .expect("Failed to run bcl");
    if !status.success() {
        let _ = fs::remove_dir_all(&work_dir);
        std::process::exit(status.code().unwrap_or(1));
    }

    let ll_path = work_dir.join("output.ll");
    let raw_ir = fs::read_to_string(&ll_path).expect("Missing output.ll");
    fs::write(&ll_path, patch_ir(&raw_ir, &map)).unwrap();

    let abs_out = if output_path.is_absolute() {
        output_path
    } else {
        env::current_dir().unwrap().join(output_path)
    };

    if wasm {
        let result = link_wasm(&work_dir, &ll_path, &abs_out);
        let _ = fs::remove_dir_all(&work_dir);
        if let Err(msg) = result {
            eprintln!("bangc: {msg}");
            std::process::exit(1);
        }
        println!("Built {} (WebAssembly)", abs_out.display());
        println!("Browsers won't load .wasm from file://, so serve the folder, e.g.:");
        println!("  python3 -m http.server   # then open http://localhost:8000/{}",
            abs_out.file_name().and_then(|n| n.to_str()).unwrap_or("bangla.html"));
        return;
    }

    let link = Command::new("clang")
        .arg(&ll_path)
        .arg(&rt_lib)
        .arg(&raylib_a)
        .args(["-lGL", "-lm", "-lpthread", "-ldl", "-lrt", "-lX11", "-o"])
        .arg(&abs_out)
        .status()
        .expect("Failed to run clang");

    let _ = fs::remove_dir_all(&work_dir);
    if !link.success() {
        std::process::exit(link.code().unwrap_or(1));
    }
    println!("Built {}", abs_out.display());
}
