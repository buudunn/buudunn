use crate::utils::*;
use wasm_bindgen::prelude::*;
use comma::parse_command;
use web_sys::CanvasRenderingContext2d;
use std::{sync::Mutex, collections::HashMap, any::Any, future::Future, sync::Arc};
use once_cell::sync::Lazy;
use eval::eval;
//use url::{Url, ParseError};


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
struct CmdContainer<'a, F: ?Sized + 'a>
where
    F: Fn(Vec<String>, &CanvasRenderingContext2d) -> Box<dyn Future<Output = Result<JsValue, JsValue>>>,
{
    func: &'a F,
}

impl<F> CmdContainer<'static, F>
where
    F: Fn(Vec<String>, &CanvasRenderingContext2d) -> Result<JsValue, JsValue>,
{
    fn new(func: F) -> CmdContainer<'static, F> {
        CmdContainer { func: &func }
    }
}

trait CmdCaller {
    fn call(&self, args: Vec<String>, context: &CanvasRenderingContext2d) -> Box<dyn Future<Output = Result<JsValue, JsValue>>>;
}

impl<F> CmdCaller for CmdContainer<'static, F>
where
    F: Fn(Vec<String>, &CanvasRenderingContext2d) -> Box<dyn Future<Output = Result<JsValue, JsValue>>>,
{
    async fn call(&self, args: Vec<String>, context: &CanvasRenderingContext2d) -> Box<dyn Future<Output = Result<JsValue, JsValue>>> {
        // Call the function inside the CmdContainer
        (self.func)(args, &context)
    }
}

pub static USER: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new("guest"));
pub static HOST: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new("local"));
pub static CWD: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new("~/"));

static COMMANDS: Lazy<Mutex<HashMap<String, Box<dyn CmdCaller + Send + Sync>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static COMMANDS_HELP: Lazy<Mutex<HashMap<String, Vec<&str>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn init_cmd() {
    let mut map = COMMANDS.lock().unwrap();
    let mut help_map = COMMANDS_HELP.lock().unwrap();
    map.insert("help".to_string(), Box::new(CmdContainer { func: &help }));
    help_map.insert("help".to_string(), vec!("Displays help.", "\nUsage: help [?subcommand]"));
    /*map.insert("echo".to_string(), Arc::new(echo));
    help_map.insert("echo".to_string(), vec!("Prints input to the console.", "\nUsage: echo \"string\""));
    map.insert("calc".to_string(), Arc::new(calc));
    help_map.insert("calc".to_string(), vec!("Performs operations on 2 or more numbers.", "\nUsage: calc [operation] [number 1, 2, 3...]"));
    map.insert("evl".to_string(), Arc::new(evl));
    help_map.insert("evl".to_string(), vec!("Evaluates an expression.", "\nUsage: evl \"expression\""));
    //map.insert("import".to_string(), |arg, ctx| import(arg, ctx));
    //help_map.insert("import".to_string(), "Imports remote commands. Internet required.".to_string());
    map.insert("abacus".to_string(), Box::new(abacus));
    help_map.insert("abacus".to_string(), vec!("Advanced mathematical operations.", "Implements multiple meval.\nUsage: abacus [operation] \"args\""));*/
    
    drop(map);
    drop(help_map);
}

#[wasm_bindgen]
pub async fn pass_cmd(cmd_str: &str, context: &CanvasRenderingContext2d) -> Result<JsValue, JsValue> {
    lock_input();
    if cmd_str.trim().is_empty() {return Ok(false.into());}
    if let Some(mut args) = parse_command(cmd_str) {
    let cmd = args.remove(0);

    draw_text("\n", &context);
    let cmds = COMMANDS.lock().unwrap();
    
    if let Some(command) = cmds.get(&cmd.to_string().to_lowercase()) {
        async {
            let _ = (*command).call(args, &context);
        }.await;
        drop(cmds);
    } else {
        draw_text(r#"\#FFC0C0Unrecognized command. Type 'help' for a list of commands."#, &context);
        return Ok(true.into());
    }
} else {
    draw_text(r#"
\#FFC0C0Error parsing arguments. Is there an end quote missing?"#, &context)
}

Ok(true.into())
}

#[wasm_bindgen]
pub async fn help(args: Vec<String>, context: &CanvasRenderingContext2d) -> Result<JsValue, JsValue> {
    let cmds_help = COMMANDS_HELP.lock().unwrap();

    if let Some(element) = args.get(0) {
        match element.as_str() {
            "about" => {
                draw_text(r#"TODO"#, &context);
            },
            _ => {
                if let Some(help) = cmds_help.get(element) {
                    draw_text(&format!("↳ {} - {}\n", element, help.get(0).expect("couldn't get help")), &context);
                    for i in 0..help.len() {
                        draw_text(&format!("↳ {} - {}\n", element, help.get(i).expect("couldn't get help")), &context);
                    }
                } else {
                    draw_text(r#"\#FFC0C0Unrecognized command/subcommand. Valid subcommands are 'about'."#, &context);
                }
            }
        }
    } else {
        cmds_help.iter().for_each(|(command, _)| {
            if let Some(help) = cmds_help.get(command) {
            draw_text(&format!("↳ {} - {}\n", command, help.get(0).expect("couldn't get help")), &context);
        }
    });
}
    drop(cmds_help);

    Ok(true.into())
}

#[wasm_bindgen]
pub async fn calc(args: Vec<String>, context: &CanvasRenderingContext2d) -> Result<JsValue, JsValue> {    if let Some(_element) = args.get(0) {
    match args[0].to_lowercase().as_str() {
        "add" => {
            if let Some(_element) = args.get(1) {
            let mut number: f64 = 0.0;
            for i in 1..args.len() {
                if let Ok(curr_number) = args[i].to_string().parse::<f64>() {
                    number += curr_number;
                } else {
                    draw_text(r#"\#FFC0C0One or more arguments are not a number."#, &context);
                    return Ok(true.into());
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
                        return Ok(true.into());
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
                return Ok(true.into());
            }

                for i in 2..args.len() {
                    if let Ok(curr_number) = args[i].to_string().parse::<f64>() {
                        number = number * curr_number;
                    } else {
                        draw_text(r#"\#FFC0C0One or more arguments are not a number."#, &context);
                        return Ok(true.into());
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
                return Ok(true.into());
            }

                for i in 2..args.len() {
                    if let Ok(curr_number) = args[i].to_string().parse::<f64>() {
                        number = number / curr_number;
                    } else {
                        draw_text(r#"\#FFC0C0One or more arguments are not a number."#, &context);
                        return Ok(true.into());
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
Ok(true.into())
}

#[wasm_bindgen]
pub async fn echo(args: Vec<String>, context: &CanvasRenderingContext2d) -> Result<JsValue, JsValue> {
    draw_text(&format!("{}", args.join(" ")), &context);
    Ok(true.into())
}

#[wasm_bindgen]
pub async fn evl(args: Vec<String>, context: &CanvasRenderingContext2d) -> Result<JsValue, JsValue> {
    if let Some(element) = args.get(0) {
        match eval(element) {
            Ok(value) => draw_text(&format!("{}", value), &context),
            Err(err) => draw_text(&format!("\\#FFC0C0Error in evaluating expression: {}", err), &context)
        }
        
    } else {
        draw_text(r#"\#FFC0C0No expression given. Make sure it's wrapped in quotes."#, &context);
    }
    Ok(true.into())
}
/*
fn import(args: Vec<String>, context: &CanvasRenderingContext2d) -> Result<(), Box<dyn std::error::Error>> {
    draw_text("Sorry! This isn't finished yet!", context);
    if let Some(element) = args.get(0) {
        match Url::parse(element) {
    Ok(_someurl) => {
    let resp = reqwest::get(element/*"https://httpbin.org/ip"*/)?
        .json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
    },
    Err(err) => Ok(draw_text(&format!("\\#FFC0C0Error in parsing URL: {}", err), &context))
    }
} else {
    Ok(draw_text(r#"\#FFC0C0No URL given. Make sure it's wrapped in quotes."#, &context))
}
}*/

#[wasm_bindgen]
pub async fn abacus(args: Vec<String>, context: &CanvasRenderingContext2d) -> Result<JsValue, JsValue> {
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
            /*"linear_solve" => {
                if let Some(expr) = args.get(1) {
                    // Parse the input string to extract coefficients and constants
    let coefficients: HashMap<String, f64> = parse_coefficients(expr);

    // Set up the linear programming problem
    let mut objective: Expression = Default::default();
    let mut vars = variables!();
    let varmap: HashMap<String, Variable> = coefficients.keys().map(|var| (var.clone(), vars.add(variable().min(f64::NEG_INFINITY).max(f64::INFINITY)))).collect();
    for (variable, coefficient) in coefficients.iter() {
        let var = varmap[variable];
        objective += var * *coefficient;
    }
    // Solve the linear programming problem
    match vars.maximise(objective).using(default_solver).solve() {
        Ok(solution) => {
            for (varb, var) in varmap.iter() {
                draw_text(&format!("{} = {}", varb, solution.value(*var)), &context);
            }
        },
        Err(err) => draw_text(&format!("\\#FFC0C0Error in parsing linear equation: {}", err), &context)
    }
                } else {
                    draw_text(r#"\#FFC0C0No expression given! Is it wrapped in quotes?"#, &context);
                }
            },*/
            &_ => draw_text(r#"\#FFC0C0Invalid operation."#, &context)
        }
    } else {
        draw_text(r#"\#FFC0C0No operation given."#, &context);
    }
    Ok(true.into())
}
/*
fn parse_coefficients(input_string: &str) -> HashMap<String, f64> {
    let mut map: HashMap<String, f64> = HashMap::new();
    let terms: Vec<&str> = input_string.split(|c| c == '+' || c == '=').collect();
    for term in terms {
        let cleaned_term = term.trim().replace(" ", "");
        let mut parts: Vec<&str> = cleaned_term.splitn(2, char::is_alphabetic).collect();
        if parts.iter().all(|s| s.is_empty()) {parts = vec![&cleaned_term]}
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
}*/