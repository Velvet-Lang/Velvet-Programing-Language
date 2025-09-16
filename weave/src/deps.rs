deps.rs
use std::fs;
use std::io::{self, Write, Read};
use std::process::Command;
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::stdout;

pub fn install_dep(lib: &str) {
    let list_url = "https://raw.githubusercontent.com/Velvet-Lang/Velvet-Programing-Language/main/weave/library/ALL-LIST.weave";
    let tmp_file = "/tmp/ALL-LIST.weave";

    // Download the list using curl
    let curl_status = Command::new("curl")
        .args(["-o", tmp_file, list_url])
        .status()
        .expect("Failed to execute curl");

    if !curl_status.success() {
        stdout()
            .execute(SetForegroundColor(Color::Red))
            .unwrap()
            .execute(Print("Failed to download library list\n"))
            .unwrap()
            .execute(ResetColor)
            .unwrap();
        return;
    }

    // Read the downloaded file
    let mut file = fs::File::open(tmp_file).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    // Parse the content
    let mut found = false;
    for line in content.lines() {
        let parts: Vec<&str> = line.split("===>").map(|s| s.trim()).collect();
        if parts.len() == 2 && parts[0] == lib {
            let repo_url = parts[1];

            // Get home directory
            let home = std::env::var("HOME").expect("HOME environment variable not set");
            let velvet_lib_dir = format!("{}/.Velvet-Lib", home);
            fs::create_dir_all(&velvet_lib_dir).unwrap();

            let target_dir = format!("{}/{}", velvet_lib_dir, lib);

            // Check if already exists
            if fs::metadata(&target_dir).is_ok() {
                stdout()
                    .execute(SetForegroundColor(Color::Yellow))
                    .unwrap()
                    .execute(Print(format!("Library {} already installed at {}\n", lib, target_dir)))
                    .unwrap()
                    .execute(ResetColor)
                    .unwrap();
                found = true;
                break;
            }

            // Clone the repository
            let clone_status = Command::new("git")
                .args(["clone", repo_url, &target_dir])
                .status()
                .expect("Failed to execute git");

            if clone_status.success() {
                stdout()
                    .execute(SetForegroundColor(Color::Green))
                    .unwrap()
                    .execute(Print(format!("Installed {} at {}\n", lib, target_dir)))
                    .unwrap()
                    .execute(ResetColor)
                    .unwrap();
            } else {
                stdout()
                    .execute(SetForegroundColor(Color::Red))
                    .unwrap()
                    .execute(Print(format!("Failed to clone {}\n", lib)))
                    .unwrap()
                    .execute(ResetColor)
                    .unwrap();
            }

            found = true;
            break;
        }
    }

    if !found {
        stdout()
            .execute(SetForegroundColor(Color::Red))
            .unwrap()
            .execute(Print(format!("Library {} not found in ALL-LIST.weave\n", lib)))
            .unwrap()
            .execute(ResetColor)
            .unwrap();
    }

    // Optional: Remove tmp file
    let _ = fs::remove_file(tmp_file);
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
