// Import (via `use`) the `fmt` module to make it available.
use rustyline::DefaultEditor;

mod environment;
mod evaluation;
mod printer;
mod reader;
use environment::MalEnv;

mod error;
use error::MalError;

mod types;
use types::MalVal;

fn fn_op(func: &str, args: &[i64]) -> i64 {
    match func {
        "+" => args.iter().sum(),
        "-" => args[1..].iter().fold(args[0], |acc, x| acc - x),
        "*" => args.iter().product(),
        "/" => args[1..].iter().fold(args[0], |acc, x| acc / x),
        _ => panic!("Unknown operator: {}", func),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // `()` can be used when no completer is required
    let mut rl = DefaultEditor::new()?;
    if rl.load_history(".mal_history.txt").is_err() {
        println!("No previous history.");
    }

    let mut env = MalEnv::new();

    env.put(
        "+",
        MalVal::Function(|args| {
            Ok(MalVal::Int(fn_op(
                "+",
                &args
                    .iter()
                    .map(|x| if let MalVal::Int(v) = x { *v } else { 0 })
                    .collect::<Vec<_>>(),
            )))
        }),
    );

    env.put(
        "-",
        MalVal::Function(|args| {
            Ok(MalVal::Int(fn_op(
                "-",
                &args
                    .iter()
                    .map(|x| if let MalVal::Int(v) = x { *v } else { 0 })
                    .collect::<Vec<_>>(),
            )))
        }),
    );

    env.put(
        "*",
        MalVal::Function(|args| {
            Ok(MalVal::Int(fn_op(
                "*",
                &args
                    .iter()
                    .map(|x| if let MalVal::Int(v) = x { *v } else { 0 })
                    .collect::<Vec<_>>(),
            )))
        }),
    );

    env.put(
        "/",
        MalVal::Function(|args| {
            Ok(MalVal::Int(fn_op(
                "/",
                &args
                    .iter()
                    .map(|x| if let MalVal::Int(v) = x { *v } else { 0 })
                    .collect::<Vec<_>>(),
            )))
        }),
    );

    // REPL
    while printer::print(evaluation::eval(reader::read(&mut rl, "user> "), &mut env)) {
        continue;
    }

    let _ = rl.save_history(".mal_history.txt");

    Ok(())
}
