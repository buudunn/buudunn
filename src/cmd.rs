use crate::utils::*;
use wasm_bindgen::prelude::*;
use comma::parse_command;
use web_sys::CanvasRenderingContext2d;

pub fn pass_cmd(cmd_str: &str, context: &CanvasRenderingContext2d) {
    let args = parse_command(cmd_str);
    draw_text("\n", &context);
    draw_text(&format!("{:#?}", args), &context)
}