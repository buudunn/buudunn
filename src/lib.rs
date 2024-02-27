mod utils;
mod cmd;
use utils::{ draw_text, backspace, set_canvas_size };

use std::f64;
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

#[wasm_bindgen(start)]
fn start() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    set_canvas_size(&canvas, &window);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.set_fill_style(&JsValue::from_str("#000000"));
    context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    context.set_fill_style(&JsValue::from_str("#FFFFFF"));
    context.set_stroke_style(&JsValue::from_str("#FFFFFF"));
    context.set_font("14px Gohu");

    let welcome_text =
r#" ____                  _                   
| __ ) _   _ _   _  __| |_   _ _ __  _ __
|  _ \| | | | | | |/ _` | | | | '_ \| '_ \
| |_) | |_| | |_| | (_| | |_| | | | | | | |
|____/ \__,_|\__,_|\__,_|\__,_|_| |_|_| |_|
                                           
Welcome to \#D8BFD8Buudunn\#FFFFFF! This is an open-sourced, web-based
mockup of the terminal emulator."#;

    draw_text(welcome_text, &context);

    let closure = Closure::wrap(
        Box::new(move |event: web_sys::KeyboardEvent| {
            event.prevent_default();
            let key = event.key();

            if key.len() == 1 && !event.ctrl_key() && !event.alt_key() && !event.meta_key() {
                draw_text(&key, &context);
            } else if key == "Enter" {
                draw_text("\n", &context);
            } else if key == "Backspace" {
                backspace(&context);
            }
        }) as Box<dyn FnMut(_)>
    );
    window.set_onkeydown(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}
