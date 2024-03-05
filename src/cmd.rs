use crate::utils::*;
use wasm_bindgen::prelude::*;
use comma::parse_command;
use web_sys::CanvasRenderingContext2d;
use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;
use eval::eval;
use good_lp::{default_solver, SolverModel, Solution, Expression, Variable, variable, variables};
//use url::{Url, ParseError};
//use wasm_bindgen_futures::JsFuture;
//use web_sys::{Request, RequestInit, RequestMode, Response};

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
    map.insert("help".to_string(), |arg, ctx| help(arg, ctx));
    help_map.insert("help".to_string(), "Displays help.".to_string());
    map.insert("echo".to_string(), |arg, ctx| echo(arg, ctx));
    help_map.insert("echo".to_string(), "Prints input to the console.".to_string());
    map.insert("calc".to_string(), |arg, ctx| calc(arg, ctx));
    help_map.insert("calc".to_string(), "Performs operations on 2 or more numbers.".to_string());
    map.insert("evl".to_string(), |arg, ctx| evl(arg, ctx));
    help_map.insert("evl".to_string(), "Evaluates an expression.".to_string());
    //map.insert("import".to_string(), |arg, ctx| import(arg, ctx));
    //help_map.insert("import".to_string(), "Imports remote commands. Internet required.".to_string());
    map.insert("abacus".to_string(), |arg, ctx| abacus(arg, ctx));
    help_map.insert("abacus".to_string(), "Advanced mathematical operations. Implements multiple libraries, including meval and nalgebra.".to_string());

    drop(map);
    drop(help_map);
}

pub fn pass_cmd(cmd_str: &str, context: &CanvasRenderingContext2d) {
    lock_input();
    if cmd_str.trim().is_empty() {return}
    if let Some(mut args) = parse_command(cmd_str) {
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
} else {
    draw_text(r#"
\#FFC0C0Error parsing arguments. Is there an end quote missing?"#, &context)
}
    
}

fn help(args: Vec<String>, context: &CanvasRenderingContext2d) {
    let cmds = COMMANDS.lock().unwrap();
    let cmds_help = COMMANDS_HELP.lock().unwrap();

    if let Some(element) = args.get(0) {
        match element.as_str() {
            "about" => {
                draw_text(r#"TODO"#, &context);
            },
            _ => {
                if let Some(help) = cmds_help.get(element) {
                    draw_text(&format!("↳ {} - {}\n", element, help), &context);
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

fn evl(args: Vec<String>, context: &CanvasRenderingContext2d) {
    if let Some(element) = args.get(0) {
        match eval(element) {
            Ok(value) => draw_text(&format!("{}", value), &context),
            Err(err) => draw_text(&format!("\\#FFC0C0Error in evaluating expression: {}", err), &context)
        }
        
    } else {
        draw_text(r#"\#FFC0C0No expression given. Make sure it's wrapped in quotes."#, &context);
    }
}

/*fn import(args: Vec<String>, context: &CanvasRenderingContext2d) {
    draw_text("Sorry! This isn't finished yet!", context);
    if let Some(element) = args.get(0) {
        match Url::parse(args.get(0).expect("error")) {
    Ok(_someurl) => {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&args.get(0).expect("error"), &opts);

    let window = web_sys::window().unwrap();
    let resp_value = window.fetch_with_str(&args.get(0).expect("error"));

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = resp.json().expect("error").expect("error");
    },
    Err(err) => draw_text(&format!("\\#FFC0C0Error in parsing URL: {}", err), &context)
    }
} else {
    draw_text(r#"\#FFC0C0No URL given. Make sure it's wrapped in quotes."#, &context);
}
}*/

fn abacus(args: Vec<String>, context: &CanvasRenderingContext2d) {
    if let Some(_elem) = args.get(0) {
        match args.get(0).expect("error").as_str() {
            "eval" => {
                if let Some(expr) = args.get(1) {
                    match meval::eval_str(expr) {
                        Ok(value) => draw_text(&format!("{}", value), &context),
                        Err(err) => draw_text(&format!("\\#FFC0C0Couldn't evaluate expression: {}", err), &context)
                    }
                } else {
                    draw_text(r#"\#FFC0C0No expression given! Is it wrapped in quotes?"#, &context);
                }
            },
            "solve" => {
                if let Some(expr) = args.get(1) {
                    // Parse the input string to extract coefficients and constants
    let coefficients: HashMap<String, f64> = parse_coefficients(expr);

    // Set up the linear programming problem
    let mut objective: Expression = Default::default();
    let mut vars = variables!();
    let varmap: HashMap<String, Variable> = coefficients.keys().map(|var| (var.clone(), vars.add(variable()))).collect();
    for (variable, coefficient) in coefficients.iter() {
        let var = varmap[variable];
        objective += var * *coefficient;
    }
    // Solve the linear programming problem
    let solution = vars.maximise(objective).using(default_solver).solve().unwrap();

    // Print the solutions for each variable
    for (variable, var) in varmap.iter() {
        draw_text(&format!("{} = {}", variable, solution.value(*var)), &context);
    }
                } else {
                    draw_text(r#"\#FFC0C0No expression given! Is it wrapped in quotes?"#, &context);
                }
            },
            &_ => draw_text(r#"\#FFC0C0Invalid operation."#, &context)
        }
    } else {
        draw_text(r#"\#FFC0C0No operation given."#, &context);
    }
}

fn parse_coefficients(input_string: &str) -> HashMap<String, f64> {
    let mut map: HashMap<String, f64> = HashMap::new();
    let terms: Vec<&str> = input_string.split(|c| c == '+' || c == '=').collect();
    console_log!("{:?}", terms);
    for term in terms {
        let cleaned_term = term.trim().replace(" ", "");
        console_log!("{:?}", cleaned_term);
        let mut parts: Vec<&str> = cleaned_term.splitn(2, char::is_alphabetic).collect();
        if parts.iter().all(|s| s.is_empty()) {parts = vec![&cleaned_term]}
        console_log!("{:?}", parts);
        if parts.len() == 2 {
            let varb = parts[0].chars().next().unwrap().to_string();
            let coefficient = parts[0].parse().unwrap_or(1.0);
            let current_coefficient = map.entry(varb.clone()).or_insert(0.0);
            *current_coefficient += coefficient;
        } else {
            if cleaned_term.chars().all(|c| c.is_alphabetic()) {
                let varb = parts[0].chars().next().unwrap().to_string();
                let coefficient = 1.0;
                let current_coefficient = map.entry(varb.clone()).or_insert(0.0);
                *current_coefficient += coefficient;
            } else {
            let constant: f64 = cleaned_term.parse().unwrap();
            let current_constant = map.entry("constant".to_string()).or_insert(0.0);
            *current_constant += constant;
            
        }
        }
    }
    map
}