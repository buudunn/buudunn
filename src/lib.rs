mod utils;
mod cmd;
use utils::*;
use cmd::*;
use std::f64;
use wasm_bindgen::prelude::*;
use console_error_panic_hook;
use std::panic;
use wasm_bindgen::JsValue;

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
async fn start() {
    lock_input();
    panic::set_hook(Box::new(console_error_panic_hook::hook));
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
    let map = init_cmd();

    let welcome_text = 
r#" ____                  _                   
| __ ) _   _ _   _  __| |_   _ _ __  _ __
|  _ \| | | | | | |/ _` | | | | '_ \| '_ \
| |_) | |_| | |_| | (_| | |_| | | | | | | |
|____/ \__,_|\__,_|\__,_|\__,_|_| |_|_| |_|
v0.1.0

Welcome to \#C8A2C8Buudunn\#FFFFFF! This is an open-sourced, web-based mockup of the terminal emulator.
To learn more, type \#FFFF00'help about'\#FFFFFF. To get a list of commands, type \#FFFF00'help'.

"#;

    let user = USER.lock().unwrap().clone();
    let host = HOST.lock().unwrap().clone();
    let cwd = CWD.lock().unwrap().clone();
    draw_text(welcome_text, &context);
    draw_text(&format!("\\#90EE90{}@{}: \\#FFFFFF\\#ADD8E6{}\\#FFFFFF \\#FFFF00$ \\#FFFFFF", user, host, cwd), &context);
    lock_cursor_here();
    unlock_input();
}

#[wasm_bindgen]
pub async fn keydownhandler(event: web_sys::KeyboardEvent, context: &web_sys::CanvasRenderingContext2d) {
    event.prevent_default();

            let key = event.key();

            if !get_is_input_locked() {
            if key.len() == 1 && !event.ctrl_key() && !event.alt_key() && !event.meta_key() {
                draw_text(&key, &context);
                add_to_cmd_bank(&key);
            } else if key == "Enter"  {
                if event.shift_key() {
                    draw_text("\n", &context);
                    add_to_cmd_bank("\n");
                } else {
                    let cmd = &get_cmd_bank();
                    clear_cmd_bank();
                    let _ = pass_cmd(cmd, &context).await;
                    draw_text("\n", &context);
                    let user = USER.lock().unwrap().clone();
                    let host = HOST.lock().unwrap().clone();
                    let cwd = CWD.lock().unwrap().clone();
                    draw_text(&format!("\\#90EE90{}@{}: \\#FFFFFF\\#ADD8E6{}\\#FFFFFF \\#FFFF00$ \\#FFFFFF", user, host, cwd), &context);
                    lock_cursor_here();
                    unlock_input();
                }
            } else if key == "Backspace" {
                backspace(&context);
                remove_last_from_cmd_bank();
            }
        }
}