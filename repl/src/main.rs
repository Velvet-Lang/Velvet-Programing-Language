use pest::Parser;
use pest_derive::Parser;
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use rustyline::Editor;
use std::io::stdout;

#[derive(Parser)]
#[grammar = "../../core/velvet.pest"]
struct VelvetParser;

fn main() {
    let mut rl = Editor::<()>::new().unwrap();
    stdout()
        .execute(SetForegroundColor(Color::Green))
        .unwrap()
        .execute(Print("Velvet REPL (type 'exit' to quit)\n"))
        .unwrap()
        .execute(ResetColor)
        .unwrap();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line == "exit" {
                    break;
                }
                match VelvetParser::parse(Rule::program, &line) {
                    Ok(pairs) => {
                        stdout()
                            .execute(SetForegroundColor(Color::Magenta))
                            .unwrap()
                            .execute(Print(format!("Parsed: {:?}\n", pairs)))
                            .unwrap()
                            .execute(ResetColor)
                            .unwrap();
                        // TODO: Evaluate AST
                    }
                    Err(e) => {
                        stdout()
                            .execute(SetForegroundColor(Color::Red))
                            .unwrap()
                            .execute(Print(format!("Error: {}\n", e)))
                            .unwrap()
                            .execute(ResetColor)
                            .unwrap();
                    }
                }
            }
            Err(_) => break,
        }
    }
}
