use std::fs;
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::stdout;

pub fn run_dev() {
    stdout()
        .execute(SetForegroundColor(Color::Cyan))
        .unwrap()
        .execute(Print("Running in dev mode (text-based, no compilation)...\n"))
        .unwrap()
        .execute(ResetColor)
        .unwrap();
    for entry in fs::read_dir("src").unwrap() {
        let path = entry.unwrap().path();
        if path.extension().map_or(false, |ext| ext == "vel") {
            let content = fs::read_to_string(&path).unwrap();
            stdout()
                .execute(SetForegroundColor(Color::Magenta))
                .unwrap()
                .execute(Print(format!("Simulating {}:\n{}\n", path.display(), content)))
                .unwrap()
                .execute(ResetColor)
                .unwrap();
        }
    }
}
