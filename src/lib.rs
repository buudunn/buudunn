use std::f64;
use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;
use std::sync::Mutex;
use gloo::{events::EventListener, timers::callback::Timeout};

lazy_static! {
    static ref CURSOR_X: Mutex<f64> = Mutex::new(10.0);
    static ref CURSOR_Y: Mutex<f64> = Mutex::new(20.0);
    static ref MAX_X: Mutex<f64> = Mutex::new(0.0);
}

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

#[wasm_bindgen(start)]
fn start() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
    canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let max_x = (canvas.width() as f64) - 20.0;
    *MAX_X.lock().unwrap() = max_x;

    context.set_fill_style(&JsValue::from_str("#000000"));
    context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    context.set_fill_style(&JsValue::from_str("#FFFFFF"));
    context.set_stroke_style(&JsValue::from_str("#FFFFFF"));
    context.set_font("16px Unifont");

    let closure = Closure::wrap(
        Box::new(move |event: web_sys::KeyboardEvent| {
            event.prevent_default();
            let key = event.key();

            console_log!("Key pressed: {}", key);
            console_log!("code: {}", event.code());
            if key.len() == 1 && !event.ctrl_key() && !event.alt_key() && !event.meta_key() {
                draw_text(&key, &context);
            } else if key == "Enter" {
                draw_text("\n", &context);
            } else if key == "Backspace" {
                draw_text("BACKSPACE ACTION \n", &context)
            } else {
                draw_text(&key, &context);
            }
        }) as Box<dyn FnMut(_)>
    );
    window.add_event_listener_with_callback("keypress", closure.as_ref().unchecked_ref()).unwrap();
}

fn draw_text(text: &str, context: &web_sys::CanvasRenderingContext2d) {
    let mut cursor_x = *CURSOR_X.lock().unwrap();
    let mut cursor_y = *CURSOR_Y.lock().unwrap();
    let max_x = *MAX_X.lock().unwrap();

    if text == "\n" {
        cursor_x = 10.0;
        cursor_y += 16.0;
    } else if text == "BACKSPACE ACTION \n" {
        if cursor_x - 8.0 < 10.0 && cursor_y - 16.0 < 20.0 {
            console_log!("Backspace cannot be performed");
        } else {
            console_log!("Backspace");
            if cursor_x < 10.0 {
                cursor_x = max_x - 8.0; // Move cursor to the end of the previous line
                cursor_y -= 16.0; // Move cursor up to the previous line
            } else {
                cursor_x -= 8.0; // Move cursor left by 8 pixels
            }
            context.set_fill_style(&JsValue::from_str("#FFFFFF"));
            context.clear_rect(cursor_x, cursor_y, 8.0, 16.0); // Clear the area of the character to be deleted
            *CURSOR_X.lock().unwrap() = cursor_x; // Update the global cursor x value
            *CURSOR_Y.lock().unwrap() = cursor_y; // Update the global cursor y value
        }
        
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
    *CURSOR_X.lock().unwrap() = cursor_x; // Update the global cursor x value
    *CURSOR_Y.lock().unwrap() = cursor_y; // Update the global cursor y value
}
