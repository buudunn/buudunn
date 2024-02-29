use crate::utils::*;
use wasm_bindgen::prelude::*;
use comma::parse_command;
use web_sys::CanvasRenderingContext2d;
use lazy_static::lazy_static;
use std::{sync::Mutex, collections::HashMap, str::FromStr};
use once_cell::sync::Lazy;

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

static COMMANDS: Lazy<Mutex<HashMap<String, fn(Vec<String>, &CanvasRenderingContext2d)>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static COMMANDS_HELP: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn init_cmd() {
    let mut map = COMMANDS.lock().unwrap();
    let mut help_map = COMMANDS_HELP.lock().unwrap();
    map.insert("echo".to_string(), |arg, ctx| echo(arg, ctx));
    help_map.insert("echo".to_string(), "Prints input to the console.".to_string());
    map.insert("calc".to_string(), |arg, ctx| calc(arg, ctx));
    help_map.insert("calc".to_string(), "Performs operations on 2 or more numbers.".to_string());
    map.insert("help".to_string(), |arg, ctx| help(arg, ctx));
    help_map.insert("help".to_string(), "Displays help.".to_string());

    drop(map);
    drop(help_map);
}

pub fn pass_cmd(cmd_str: &str, context: &CanvasRenderingContext2d) {
    lock_input();
    let mut args = parse_command(cmd_str).expect("Error parsing arguments. Is there an end quote missing?");
    let cmd = args.remove(0);
    console_log!("{:?}", cmd);

    draw_text("\n", &context);
    let cmds = COMMANDS.lock().unwrap();
    if let Some(func) = cmds.get(&cmd.to_string()) {
        let func = func.to_owned();
        drop(cmds);
        func(args, &context);
    } else {
        draw_text(r#"\#FFC0C0Unrecognized command. Type 'help' for a list of commands."#, &context);
    }
}

fn help(args: Vec<String>, context: &CanvasRenderingContext2d) {
    let cmds = COMMANDS.lock().unwrap();
    let cmds_help = COMMANDS_HELP.lock().unwrap();

    if let Some(element) = args.get(0) {
        match element {
            "about" => {
                draw_text(r#"TODO"#, &context);
            },
            _ => {
                if let Some(help) = cmds_help.get(command) {
                    draw_text(&format!("↳ {} - {}\n", command, help), &context);
                } else {
                    draw_text(r#"\#FFC0C0Unrecognized command/subcommand. Valid subcommands are 'about'."#, &context);
                }
            }
        }
    } else {
    for (command, _) in cmds.iter() {
        if let Some(help) = cmds_help.get(command) {
            draw_text(&format!("↳ {} - {}\n", command, help), &context);
        }
    }
}
    drop(cmds);
    drop(cmds_help);
}

fn calc(args: Vec<String>, context: &CanvasRenderingContext2d) {
    if let Some(_element) = args.get(0) {
    match args[0].to_lowercase().as_str() {
        "add" => {
            if let Some(_element) = args.get(1) {
            let mut number: f64 = 0.0;
            for i in 1..args.len() {
                if let Ok(curr_number) = args[i].to_string().parse::<f64>() {
                    number += curr_number;
                } else {
                    draw_text(r#"\#FFC0C0One or more arguments are not a number."#, &context);
                    return;
                }
            }
            draw_text(&format!("{}", number), &context);
        } else {
            draw_text(r#"\#FFC0C0No arguments specified!"#, &context);
        }
        },
        "sub" => {
            if let Some(_element) = args.get(1) {
                let mut number: f64 = 0.0;
                for i in 1..args.len() {
                    if let Ok(curr_number) = args[i].to_string().parse::<f64>() {
                        number -= curr_number;
                    } else {
                        draw_text(r#"\#FFC0C0One or more arguments are not a number."#, &context);
                        return;
                    }
                }
                draw_text(&format!("{}", number), &context);
            } else {
                draw_text(r#"\#FFC0C0No arguments specified!"#, &context);
            }
        },
        "mul" => {
            if let Some(_element) = args.get(1) {
                let mut number: f64 = 0.0;
                if let Ok(curr_number) = args[1].to_string().parse::<f64>() {
                number = curr_number;
            } else {
                draw_text(r#"\#FFC0C0The first argument is not a number."#, &context);
                return;
            }

                for i in 2..args.len() {
                    if let Ok(curr_number) = args[i].to_string().parse::<f64>() {
                        number = number * curr_number;
                    } else {
                        draw_text(r#"\#FFC0C0One or more arguments are not a number."#, &context);
                        return;
                    }
                }
                draw_text(&format!("{}", number), &context);
            } else {
                draw_text(r#"\#FFC0C0No arguments specified!"#, &context);
            }
        },
        "div" => {
            if let Some(_element) = args.get(1) {
                let mut number: f64 = 0.0;
                if let Ok(curr_number) = args[1].to_string().parse::<f64>() {
                number = curr_number;
            } else {
                draw_text(r#"\#FFC0C0The first argument is not a number."#, &context);
                return;
            }

                for i in 2..args.len() {
                    if let Ok(curr_number) = args[i].to_string().parse::<f64>() {
                        number = number / curr_number;
                    } else {
                        draw_text(r#"\#FFC0C0One or more arguments are not a number."#, &context);
                        return;
                    }
                }
                draw_text(&format!("{}", number), &context);
            } else {
                draw_text(r#"\#FFC0C0No arguments specified!"#, &context);
            }
        },
        _ => draw_text(r#"\#FFC0C0Unrecognized operation. Valid operations are 'add', 'sub', 'mul', and 'div'."#, &context),
    }
} else {
    draw_text(r#"\#FFC0C0Missing operation. Valid operations are 'add', 'sub', 'mul', and 'div'."#, &context);
}
}

fn echo(args: Vec<String>, context: &CanvasRenderingContext2d) {
    draw_text(&format!("{}", args.join(" ")), &context);
}