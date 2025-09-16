use std::fs;
use std::io::Write;
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::stdout;

pub fn init_project() {
    fs::create_dir_all("src").unwrap();
    let mut file = fs::File::create("weave.velvet").unwrap();
    file.write_all(b"<Project Name>\n[Dependences]\n[BUILD]\n").unwrap();
    stdout()
        .execute(SetForegroundColor(Color::Green))
        .unwrap()
        .execute(Print("Project initialized with weave.velvet and src/\n"))
        .unwrap()
        .execute(ResetColor)
        .unwrap();
}

pub fn clear_project() {
    for entry in fs::read_dir(".").unwrap() {
        let path = entry.unwrap().path();
        if let Some(ext) = path.extension() {
            if ext == "o" || ext == "velvet" {
                fs::remove_file(&path).unwrap();
            }
        }
    }
    stdout()
        .execute(SetForegroundColor(Color::Green))
        .unwrap()
        .execute(Print("Project cleared\n"))
        .unwrap()
        .execute(ResetColor)
        .unwrap();
}
