use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use std::env;
use std::collections::HashMap;
use git2::Repository;
use crate::{parse_velvet, scan_project, compile_to_rust, run_velvet};

pub fn read_file(filename: &str) -> Result<String, String> {
    fs::read_to_string(filename).map_err(|e| format!("Error reading {}: {}", filename, e))
}

pub fn read_library_velvet() -> Result<Vec<(String, String)>, String> {
    let content = read_file("library.velvet")?;
    let mut deps = Vec::new();
    for line in content.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        if parts.len() == 2 {
            deps.push((parts[0].trim().to_string(), parts[1].trim().to_string()));
        }
    }
    Ok(deps)
}

pub fn add_to_library(dep: &str, repo: &str) -> Result<(), String> {
    let mut f = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("/weave/library.txt")
        .map_err(|e| format!("Error opening library.txt: {}", e))?;
    writeln!(f, "{} - {}", dep, repo).map_err(|e| format!("Error writing to library.txt: {}", e))?;
    Ok(())
}

pub fn find_repo_for_dep(dep: &str) -> Result<Option<String>, String> {
    let content = read_file("/weave/library.txt").unwrap_or_default();
    for line in content.lines() {
        let parts: Vec<&str> = line.split(" - ").collect();
        if parts.len() == 2 && parts[0] == dep {
            return Ok(Some(parts[1].to_string()));
        }
    }
    Ok(None)
}

pub fn setup_isolated_env() -> Result<(), String> {
    fs::create_dir_all("/weave").map_err(|e| format!("Error creating /weave: {}", e))?;
    File::create("/weave/library.txt").map_err(|e| format!("Error creating library.txt: {}", e))?;
    Ok(())
}

pub fn install_external(dep: &str) -> Result<(), String> {
    let cmd = format!("sudo chroot /weave apt update && apt install -y {}", dep);
    let status = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .status()
        .map_err(|e| format!("Error running install-x: {}", e))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("install-x failed with status: {}", status))
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: weave <command> [args]\nCommands: init, run, install, build, install-x, help");
        return Err("Missing command".to_string());
    }

    let cmd = &args[1];
    match cmd.as_str() {
        "init" => {
            fs::create_dir_all("src").map_err(|e| format!("Error creating src/: {}", e))?;
            let content = "[zaleznosc]\n@komentarz\n#velvet {\n5 < A\n}\n";
            fs::write("src/main.vel", content).map_err(|e| format!("Error writing src/main.vel: {}", e))?;
            println!("Project initialized with src/main.vel");
            Ok(())
        }
        "run" => {
            if args.len() < 3 {
                return Err("Missing file argument for run".to_string());
            }
            let input = read_file(&args[2])?;
            run_velvet(&input).map_err(|e| format!("Error running {}: {}", args[2], e))?;
            Ok(())
        }
        "install" => {
            if args.len() < 4 {
                return Err("Missing dep or repo argument for install".to_string());
            }
            let dep = &args[2];
            let repo = &args[3];
            add_to_library(dep, repo)?;
            let dest = format!("/weave/{}", dep);
            match Repository::clone(repo, &dest) {
                Ok(_) => {
                    println!("Installed {} from {}", dep, repo);
                    Ok(())
                }
                Err(e) => Err(format!("Error cloning {} to {}: {}", repo, dest, e)),
            }
        }
        "build" => {
            let (deps, errors) = scan_project(".").map_err(|e| format!("Error scanning project: {}", e))?;
            if !errors.is_empty() {
                for err in errors {
                    println!("{}", err);
                }
                return Err("Build failed due to scan errors".to_string());
            }

            let library_deps = read_library_velvet()?;
            fs::create_dir_all("build/lib").map_err(|e| format!("Error creating build/lib: {}", e))?;
            fs::create_dir_all("build/build-files").map_err(|e| format!("Error creating build/build-files: {}", e))?;
            fs::create_dir_all("build/linux").map_err(|e| format!("Error creating build/linux: {}", e))?;
            fs::create_dir_all("build/windows").map_err(|e| format!("Error creating build/windows: {}", e))?;
            fs::create_dir_all("build/macos").map_err(|e| format!("Error creating build/macos: {}", e))?;

            for (dep, repo) in library_deps {
                if let Some(repo_url) = find_repo_for_dep(&dep)? {
                    let lib_dest = format!("build/lib/{}", dep);
                    Repository::clone(&repo_url, &lib_dest)
                        .map_err(|e| format!("Error cloning {} to {}: {}", repo_url, lib_dest, e))?;
                }
            }

            let main_input = read_file("src/main.vel")?;
            let ast = parse_velvet(&main_input).map_err(|e| format!("Error parsing src/main.vel: {}", e))?;
            compile_to_rust(&ast, "build/build-files/main.rs", &deps)
                .map_err(|e| format!("Error compiling to Rust: {}", e))?;

            let targets = [
                ("x86_64-linux-gnu", "build/linux/velvet"),
                ("x86_64-windows-gnu", "build/windows/velvet.exe"),
                ("x86_64-macos", "build/macos/velvet"),
            ];
            for (target, output) in targets.iter() {
                let cmd = format!("zig build-exe build/build-files/main.rs -O ReleaseFast -target {} -o {}", target, output);
                let status = Command::new("sh")
                    .arg("-c")
                    .arg(&cmd)
                    .status()
                    .map_err(|e| format!("Error running zig for {}: {}", target, e))?;
                if !status.success() {
                    return Err(format!("Zig build failed for {}", target));
                }
            }

            if args.len() > 2 {
                match args[2].as_str() {
                    "appimage" => {
                        Command::new("appimagetool")
                            .arg("build/linux")
                            .arg("velvet.appimage")
                            .status()
                            .map_err(|e| format!("Error creating AppImage: {}", e))?;
                    }
                    "deb" => {
                        Command::new("checkinstall")
                            .arg("-D")
                            .arg("make")
                            .arg("install")
                            .status()
                            .map_err(|e| format!("Error creating deb: {}", e))?;
                    }
                    "rpm" => {
                        Command::new("checkinstall")
                            .arg("-R")
                            .arg("make")
                            .arg("install")
                            .status()
                            .map_err(|e| format!("Error creating rpm: {}", e))?;
                    }
                    _ => println!("Unknown build target: {}", args[2]),
                }
            }

            println!("Built binaries in /build");
            Ok(())
        }
        "install-x" => {
            if args.len() < 3 {
                return Err("Missing dep argument for install-x".to_string());
            }
            install_external(&args[2])?;
            Ok(())
        }
        "help" => {
            println!("Weave help:\n- init\n- run {file}\n- install {dep} {repo}\n- build [appimage/deb/rpm]\n- install-x {dep}\n- help");
            Ok(())
        }
        _ => {
            println!("Unknown command: {}", cmd);
            Err("Unknown command".to_string())
        }
    }
}
