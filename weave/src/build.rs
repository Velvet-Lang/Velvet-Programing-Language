use std::fs;
use std::process::Command;
use walkdir::WalkDir;
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::stdout;

pub fn build_project(options: &[&str]) {
    let vel_files: Vec<_> = WalkDir::new("src")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "vel"))
        .map(|e| e.path().to_str().unwrap().to_string())
        .collect();

    if vel_files.is_empty() {
        stdout()
            .execute(SetForegroundColor(Color::Red))
            .unwrap()
            .execute(Print("No .vel files found in src/\n"))
            .unwrap()
            .execute(ResetColor)
            .unwrap();
        return;
    }

    fs::create_dir_all("build").unwrap();
    for vel in &vel_files {
        let output = format!("build/{}.o", vel.strip_prefix("src/").unwrap());
        stdout()
            .execute(SetForegroundColor(Color::Cyan))
            .unwrap()
            .execute(Print(format!("Compiling {} to {}...\n", vel, output)))
            .unwrap()
            .execute(ResetColor)
            .unwrap();
        let status = Command::new("cargo")
            .args(&["run", "--bin", "velvetc", vel, "-o", &output])
            .current_dir("../core")
            .status()
            .unwrap();
        if !status.success() {
            stdout()
                .execute(SetForegroundColor(Color::Red))
                .unwrap()
                .execute(Print("Compilation failed\n"))
                .unwrap()
                .execute(ResetColor)
                .unwrap();
            return;
        }
    }

    stdout()
        .execute(SetForegroundColor(Color::Cyan))
        .unwrap()
        .execute(Print("Packaging with Zig...\n"))
        .unwrap()
        .execute(ResetColor)
        .unwrap();
    let status = Command::new("zig")
        .args(&["build"])
        .current_dir("..")
        .status()
        .unwrap();
    if !status.success() {
        stdout()
            .execute(SetForegroundColor(Color::Red))
            .unwrap()
            .execute(Print("Packaging failed\n"))
            .unwrap()
            .execute(ResetColor)
            .unwrap();
        return;
    }

    for opt in options {
        stdout()
            .execute(SetForegroundColor(Color::Cyan))
            .unwrap()
            .execute(Print(format!("Building {} package...\n", opt)))
            .unwrap()
            .execute(ResetColor)
            .unwrap();
        // Placeholder: Implement deb/rpm/bin/appimage packaging
    }

    stdout()
        .execute(SetForegroundColor(Color::Green))
        .unwrap()
        .execute(Print("Build complete\n"))
        .unwrap()
        .execute(ResetColor)
        .unwrap();
}
