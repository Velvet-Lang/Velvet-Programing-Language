use std::fs;
use std::io::Write;
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::stdout;

pub fn install_dep(lib: &str) {
    stdout()
        .execute(SetForegroundColor(Color::Cyan))
        .unwrap()
        .execute(Print(format!("Installing dependency {}...\n", lib)))
        .unwrap()
        .execute(ResetColor)
        .unwrap();
    // Placeholder: Download from Velvet repo
}

pub fn install_in(lib: &str) {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("weave.velvet")
        .unwrap();
    file.write_all(format!("{}\n", lib).as_bytes()).unwrap();
    stdout()
        .execute(SetForegroundColor(Color::Green))
        .unwrap()
        .execute(Print(format!("Added {} to weave.velvet\n", lib)))
        .unwrap()
        .execute(ResetColor)
        .unwrap();
}

pub fn install_o(lib: &str) {
    fs::create_dir_all("isolated_env").unwrap();
    stdout()
        .execute(SetForegroundColor(Color::Green))
        .unwrap()
        .execute(Print(format!("Installed {} in isolated_env/\n", lib)))
        .unwrap()
        .execute(ResetColor)
        .unwrap();
}

pub fn add_package(pkg: &str) {
    stdout()
        .execute(SetForegroundColor(Color::Cyan))
        .unwrap()
        .execute(Print(format!("Adding package {}...\n", pkg)))
        .unwrap()
        .execute(ResetColor)
        .unwrap();
    // Placeholder: Fetch from GitHub/SourceForge
}
