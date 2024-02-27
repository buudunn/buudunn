2use lazy_static::lazy_static;
use std::sync::Mutex;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()));
}

lazy_static! {
    static ref CURSOR_POS: Mutex<(f64, f64)> = Mutex::new((10.0, 20.0));
    static ref CURSOR_LOCK_POS: Mutex<(f64, f64)> = Mutex::new((0.0, 0.0));
    static ref IS_CURSOR_LOCKED: Mutex<bool> = Mutex::new(false);
    static ref IS_INPUT_LOCKED: Mutex<bool> = Mutex::new(false);
    static ref MAX_POS: Mutex<(f64, f64)> = Mutex::new((0.0, 0.0));
    static ref FONT_SIZE: Mutex<f64> = Mutex::new(14.0);
}

pub fn set_canvas_size(canvas: &web_sys::HtmlCanvasElement, window: &web_sys::Window) {
    canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
    canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);
    let max_x = (canvas.width() as f64) - 20.0;
    let max_y = (canvas.height() as f64) - 20.0;
    *MAX_POS.lock().unwrap() = (max_x, max_y);
}

pub fn lock_cursor(position: (f64, f64)) {
    *MAX_POS.lock().unwrap() = position;
    *IS_CURSOR_LOCKED.lock().unwrap() = true;
}

pub fn unlock_cursor() {
    *IS_CURSOR_LOCKED.lock().unwrap() = false;
}

pub fn toggle_input() {
    *IS_INPUT_LOCKED.lock().unwrap() = !*IS_INPUT_LOCKED.lock().unwrap();
}

pub fn draw_text(text: &str, context: &web_sys::CanvasRenderingContext2d) {
    if !*IS_INPUT_LOCKED.lock().unwrap() {
    let mut cursor_pos = *CURSOR_POS.lock().unwrap();
    let max_x = MAX_POS.lock().unwrap().0;
    let font_size = *FONT_SIZE.lock().unwrap();

    if text == "\n" {
        cursor_pos.0 = 10.0;
        cursor_pos.1 += font_size + 4.0;
    } else {
        for ch in text.chars() {
            if cursor_pos.0 + font_size/2.0 >= max_x {
                // Check if the cursor is about to go off screen or beyond the max_x
                cursor_pos.0 = 10.0; // Reset the cursor x
                cursor_pos.1 += font_size + 4.0; // Increase the cursor y
            }
            context.fill_text(&ch.to_string(), cursor_pos.0, cursor_pos.1).unwrap(); // Draw the character
            cursor_pos.0 += font_size/2.0; // Increment the cursor x
        }
    }
    console_log!("x: {} y: {}", cursor_pos.0, cursor_pos.1);

    *CURSOR_POS.lock().unwrap() = (cursor_pos.0, cursor_pos.1); // Update the global cursor x and y value
}
}

pub fn backspace(context: &web_sys::CanvasRenderingContext2d) {
    let mut cursor_pos = *CURSOR_POS.lock().unwrap();
    let max_x = MAX_POS.lock().unwrap().0;
    let lock_pos = CURSOR_LOCK_POS.lock().unwrap();
    let font_size = *FONT_SIZE.lock().unwrap();

    if cursor_pos.0 - font_size/2.0 < 9.9 && cursor_pos.1 - font_size+4.0 < 20.0 {
        /* can't backspace */
    } else {
        if cursor_pos.0 - font_size/2.0 < 9.0 {
            cursor_pos.0 = max_x-9.0; // Move cursor to the end of the previous line
            cursor_pos.1 -= font_size+4.0; // Move cursor up to the previous line
        } else {
            cursor_pos.0 -= font_size/2.0; // Move cursor left by 8 pixels
        }
        context.set_stroke_style(&JsValue::from_str("#000000"));
        context.set_fill_style(&JsValue::from_str("#000000"));
        context.fill_rect(cursor_pos.0, cursor_pos.1-font_size, font_size/2.0, font_size+4.0);
        context.set_stroke_style(&JsValue::from_str("#FFFFFF"));
        context.set_fill_style(&JsValue::from_str("#FFFFFF"));
        console_log!("x: {} y: {}", cursor_pos.0, cursor_pos.1);
        *CURSOR_POS.lock().unwrap() = (cursor_pos.0, cursor_pos.1); // Update the global cursor x and y value
    }
}