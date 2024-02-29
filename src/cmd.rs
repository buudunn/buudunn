use crate::utils::*;
use wasm_bindgen::prelude::*;
use comma::parse_command;
use std::collections::HashMap;
use web_sys::CanvasRenderingContext2d;
use lazy_static::lazy_static;
use std::sync::Mutex;

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

trait CommandFn: Fn(Vec<String>, &CanvasRenderingContext2d) {}
impl<T> CommandFn for T where T: Fn(Vec<String>, &CanvasRenderingContext2d) {}

lazy_static! {
    static ref COMMANDS: Mutex<HashMap<String, fn(Vec<String>, &CanvasRenderingContext2d) -> ()>> = Mutex::new(HashMap::new());
    static ref COMMANDS_HELP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub fn init_cmd() {
    let mut map = COMMANDS.lock().unwrap();
    let help_map = COMMANDS_HELP.lock().unwrap();
    map.insert("echo".to_string(), |arg, ctx| echo(arg, ctx));
    map.insert("calc".to_string(), |arg, ctx| calc(arg, ctx));
    map.insert("help".to_string(), |arg, ctx| help(arg, ctx));
}

pub fn pass_cmd(cmd_str: &str, context: &CanvasRenderingContext2d) {
    lock_input();
    let command_list = COMMANDS.lock().unwrap();
    let mut args = parse_command(cmd_str).expect("Error parsing arguments. Is there an end quote missing?");
    let cmd = args.remove(0);
    console_log!("{:?}", cmd);

    draw_text("\n", &context);

    let func = command_list.get(&cmd.to_string()).unwrap();
    drop(command_list);
    func(args, &context);
    //draw_text(r#"\#FFC0C0Unrecognized command. Type 'help' for a list of commands."#, &context);
}

fn help(args: Vec<String>, context: &CanvasRenderingContext2d) {
    let commands = COMMANDS.lock().unwrap();
    let commands_help = COMMANDS_HELP.lock().unwrap();
    for (command, _) in commands.iter() {
        if let Some(help) = commands_help.get(command) {
            draw_text(&format!(" {} - Help: {}", command, help), &context);
        }
    }
}

fn calc(args: Vec<String>, context: &CanvasRenderingContext2d) {
    draw_text(&format!("{:#?}", args), &context);
}

fn echo(args: Vec<String>, context: &CanvasRenderingContext2d) {
    draw_text(&format!("{}", args.join(" ")), &context);
}