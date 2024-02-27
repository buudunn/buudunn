use lazy_static::lazy_static;
use std::sync::Mutex;
use wasm_bindgen::JsValue;

lazy_static! {
    static ref CURSOR_POS: Mutex<(f64, f64)> = Mutex::new((10.0, 20.0));
    static ref CURSOR_LOCK_POS: Mutex<(f64, f64)> = Mutex::new((0.0, 0.0));
    static ref IS_CURSOR_LOCKED: Mutex<bool> = Mutex::new(false);
    static ref IS_INPUT_LOCKED: Mutex<bool> = Mutex::new(false);
    static ref MAX_POS: Mutex<(f64, f64)> = Mutex::new((0.0, 0.0));
}

pub fn set_canvas_size(canvas: &web_sys::HtmlCanvasElement, window: &web_sys::Window) {
    canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
    canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);
    let max_x = (canvas.width() as f64) - 20.0;
    let max_y = (canvas.height() as f64) - 20.0;
    *MAX_POS.lock().unwrap() = (max_x, max_y);
}

pub fn lock_cursor() {

}

pub fn draw_text(text: &str, context: &web_sys::CanvasRenderingContext2d) {
    if !*IS_INPUT_LOCKED.lock().unwrap() {
    let cursor_pos = *CURSOR_POS.lock().unwrap();
    let mut cursor_x = cursor_pos.0;
    let mut cursor_y = cursor_pos.1;
    let max_x = MAX_POS.lock().unwrap().0;

    if text == "\n" {
        cursor_x = 10.0;
        cursor_y += 16.0;
    } else {
        for ch in text.chars() {
            if cursor_x + 8.0 >= max_x {
                // Check if the cursor is about to go off screen or beyond the max_x
                cursor_x = 10.0; // Reset the cursor x
                cursor_y += 16.0; // Increase the cursor y
            }
            context.fill_text(&ch.to_string(), cursor_x, cursor_y).unwrap(); // Draw the character
            cursor_x += 8.0; // Increment the cursor x
        }
    }
    *CURSOR_POS.lock().unwrap() = (cursor_x, cursor_y); // Update the global cursor x and y value
}
}

pub fn backspace(context: &web_sys::CanvasRenderingContext2d) {
    let cursor_pos = *CURSOR_POS.lock().unwrap();
    let mut cursor_x = cursor_pos.0;
    let mut cursor_y = cursor_pos.1;
    let max_x = MAX_POS.lock().unwrap().0;

    if cursor_x - 8.0 < 10.0 && cursor_y - 16.0 < 20.0 {
        /* can't backspace */
    } else {
        if cursor_x < 10.0 {
            cursor_x = max_x - 8.0; // Move cursor to the end of the previous line
            cursor_y -= 16.0; // Move cursor up to the previous line
        } else {
            cursor_x -= 8.0; // Move cursor left by 8 pixels
        }
        context.set_stroke_style(&JsValue::from_str("#000000"));
        context.set_fill_style(&JsValue::from_str("#000000"));
        context.fill_rect(cursor_x, cursor_y, 8.0, -16.0); // Clear the area of the character to be deleted
        context.set_stroke_style(&JsValue::from_str("#FFFFFF"));
        context.set_fill_style(&JsValue::from_str("#FFFFFF"));
        *CURSOR_POS.lock().unwrap() = (cursor_x, cursor_y); // Update the global cursor x and y value
    }
}