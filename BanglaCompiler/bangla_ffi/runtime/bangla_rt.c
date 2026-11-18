/*
 * BangLang runtime for the WebAssembly target.
 *
 * C port of bangla_ffi/src/lib.rs so that `bangc --wasm` does not need a Rust
 * wasm toolchain (the Rust emscripten target has to match the installed emsdk
 * version, and passing `Color` by value from Rust can disagree with the C ABI).
 * Every function below has the same name, arguments and int return value as
 * the Rust version, so the IR that bangc generates links against either one.
 *
 * Only raylib's functions are declared here (no raylib.h needed).
 */
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct Color {
    unsigned char r, g, b, a;
} Color;

extern void InitWindow(int width, int height, const char *title);
extern bool WindowShouldClose(void);
extern void CloseWindow(void);
extern void SetTargetFPS(int fps);
extern void BeginDrawing(void);
extern void EndDrawing(void);
extern void ClearBackground(Color color);
extern void DrawRectangle(int x, int y, int width, int height, Color color);
extern void DrawText(const char *text, int x, int y, int fontSize, Color color);
extern bool IsKeyPressed(int key);
extern int GetRandomValue(int min, int max);

/* ---- lists (snake segments) ---- */

typedef struct {
    int *data;
    int len;
    int cap;
} List;

static List *lists = NULL;
static int list_count = 0;
static int list_cap = 0;

static List *get_list(int id) {
    if (id < 0 || id >= list_count) return NULL;
    return &lists[id];
}

int bl_add(int a, int b) { return a + b; }

int bl_list_new(void) {
    if (list_count == list_cap) {
        int new_cap = list_cap ? list_cap * 2 : 8;
        List *grown = (List *)realloc(lists, (size_t)new_cap * sizeof(List));
        if (!grown) return -1;
        lists = grown;
        list_cap = new_cap;
    }
    lists[list_count].data = NULL;
    lists[list_count].len = 0;
    lists[list_count].cap = 0;
    return list_count++;
}

int bl_list_push(int id, int val) {
    List *l = get_list(id);
    if (!l) return -1;
    if (l->len == l->cap) {
        int new_cap = l->cap ? l->cap * 2 : 16;
        int *grown = (int *)realloc(l->data, (size_t)new_cap * sizeof(int));
        if (!grown) return -1;
        l->data = grown;
        l->cap = new_cap;
    }
    l->data[l->len++] = val;
    return l->len;
}

int bl_list_get(int id, int idx) {
    List *l = get_list(id);
    if (!l || idx < 0 || idx >= l->len) return 0;
    return l->data[idx];
}

int bl_list_set(int id, int idx, int val) {
    List *l = get_list(id);
    if (!l || idx < 0 || idx >= l->len) return -1;
    l->data[idx] = val;
    return 0;
}

int bl_list_len(int id) {
    List *l = get_list(id);
    return l ? l->len : 0;
}

/* ---- raylib wrappers ---- */

static Color rgb(int r, int g, int b) {
    Color c = {(unsigned char)r, (unsigned char)g, (unsigned char)b, 255};
    return c;
}

int bl_init_window(int w, int h) {
    InitWindow(w, h, "BangLang + Raylib");
    return 0;
}

int bl_window_should_close(void) { return WindowShouldClose() ? 1 : 0; }
int bl_close_window(void) { CloseWindow(); return 0; }
int bl_set_target_fps(int fps) { SetTargetFPS(fps); return 0; }
int bl_begin_drawing(void) { BeginDrawing(); return 0; }
int bl_end_drawing(void) { EndDrawing(); return 0; }

int bl_clear_background(int r, int g, int b) {
    ClearBackground(rgb(r, g, b));
    return 0;
}

int bl_draw_rect(int x, int y, int w, int h, int r, int g, int b) {
    DrawRectangle(x, y, w, h, rgb(r, g, b));
    return 0;
}

int bl_draw_number(int val, int x, int y, int size, int r, int g, int b) {
    char txt[48];
    snprintf(txt, sizeof txt, "Score: %d", val);
    DrawText(txt, x, y, size, rgb(r, g, b));
    return 0;
}

int bl_is_key_pressed(int key) { return IsKeyPressed(key) ? 1 : 0; }
int bl_random(int min, int max) { return GetRandomValue(min, max); }
