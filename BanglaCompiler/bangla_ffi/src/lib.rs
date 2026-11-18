use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::sync::Mutex;

static LISTS: Mutex<Vec<Vec<i32>>> = Mutex::new(Vec::new());

#[repr(C)]
#[derive(Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

unsafe extern "C" {
    fn InitWindow(width: c_int, height: c_int, title: *const c_char);
    fn WindowShouldClose() -> bool;
    fn CloseWindow();
    fn SetTargetFPS(fps: c_int);
    fn BeginDrawing();
    fn EndDrawing();
    fn ClearBackground(color: Color);
    fn DrawRectangle(posX: c_int, posY: c_int, width: c_int, height: c_int, color: Color);
    fn DrawText(text: *const c_char, posX: c_int, posY: c_int, fontSize: c_int, color: Color);
    fn IsKeyPressed(key: c_int) -> bool;
    fn GetRandomValue(min: c_int, max: c_int) -> c_int;
}

#[unsafe(no_mangle)]
pub extern "C" fn bl_add(a: i32, b: i32) -> i32 {
    a + b
}

// --- List helpers (for snake segments) ---

#[unsafe(no_mangle)]
pub extern "C" fn bl_list_new() -> i32 {
    let mut lists = LISTS.lock().unwrap();
    let id = lists.len() as i32;
    lists.push(Vec::new());
    id
}

#[unsafe(no_mangle)]
pub extern "C" fn bl_list_push(id: i32, val: i32) -> i32 {
    let mut lists = LISTS.lock().unwrap();
    if let Some(list) = lists.get_mut(id as usize) {
        list.push(val);
        list.len() as i32
    } else {
        -1
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn bl_list_get(id: i32, idx: i32) -> i32 {
    let lists = LISTS.lock().unwrap();
    lists
        .get(id as usize)
        .and_then(|l| l.get(idx as usize))
        .copied()
        .unwrap_or(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn bl_list_set(id: i32, idx: i32, val: i32) -> i32 {
    let mut lists = LISTS.lock().unwrap();
    if let Some(slot) = lists.get_mut(id as usize).and_then(|l| l.get_mut(idx as usize)) {
        *slot = val;
        0
    } else {
        -1
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn bl_list_len(id: i32) -> i32 {
    let lists = LISTS.lock().unwrap();
    lists.get(id as usize).map(|l| l.len() as i32).unwrap_or(0)
}

// --- Raylib wrappers ---

#[unsafe(no_mangle)]
pub extern "C" fn bl_init_window(w: i32, h: i32) -> i32 {
    let title = CString::new("BangLang + Raylib").unwrap();
    unsafe { InitWindow(w, h, title.as_ptr()) };
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn bl_window_should_close() -> i32 {
    if unsafe { WindowShouldClose() } { 1 } else { 0 }
}

#[unsafe(no_mangle)]
pub extern "C" fn bl_close_window() -> i32 {
    unsafe { CloseWindow() };
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn bl_set_target_fps(fps: i32) -> i32 {
    unsafe { SetTargetFPS(fps) };
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn bl_begin_drawing() -> i32 {
    unsafe { BeginDrawing() };
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn bl_end_drawing() -> i32 {
    unsafe { EndDrawing() };
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn bl_clear_background(r: i32, g: i32, b: i32) -> i32 {
    unsafe { ClearBackground(Color { r: r as u8, g: g as u8, b: b as u8, a: 255 }) };
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn bl_draw_rect(x: i32, y: i32, w: i32, h: i32, r: i32, g: i32, b: i32) -> i32 {
    unsafe { DrawRectangle(x, y, w, h, Color { r: r as u8, g: g as u8, b: b as u8, a: 255 }) };
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn bl_draw_number(val: i32, x: i32, y: i32, size: i32, r: i32, g: i32, b: i32) -> i32 {
    let txt = CString::new(format!("Score: {val}")).unwrap();
    unsafe { DrawText(txt.as_ptr(), x, y, size, Color { r: r as u8, g: g as u8, b: b as u8, a: 255 }) };
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn bl_is_key_pressed(key: i32) -> i32 {
    if unsafe { IsKeyPressed(key) } { 1 } else { 0 }
}

#[unsafe(no_mangle)]
pub extern "C" fn bl_random(min: i32, max: i32) -> i32 {
    unsafe { GetRandomValue(min, max) }
}
