use clap::{Parser, Subcommand};
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::stdout;

mod project;
mod deps;
mod build;
mod run;

#[derive(Parser)]
#[command(name = "weave", about = "Weave: Velvet Build Tool", version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Clear,
    Help,
    Install { lib: String },
    InstallIn { lib: String },
    InstallO { lib: String },
    Add { pkg: String },
    Build {
        #[arg(long)] deb: bool,
        #[arg(long)] rpm: bool,
        #[arg(long)] bin: bool,
        #[arg(long)] appimage: bool,
    },
    Run,
    Test,
    Fmt,
    Doc,
    Repl,
}

fn cyber_print(msg: &str) {
    stdout()
        .execute(SetForegroundColor(Color::Green))
        .unwrap()
        .execute(Print(format!("{}\n", msg)))
        .unwrap()
        .execute(ResetColor)
        .unwrap();
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            cyber_print("Initializing Velvet project...");
            project::init_project();
        }
        Commands::Clear => {
            cyber_print("Clearing project...");
            project::clear_project();
        }
        Commands::Help => {
            Cli::parse().print_help().unwrap();
        }
        Commands::Install { lib } => {
            cyber_print(&format!("Installing {}...", lib));
            deps::install_dep(&lib);
        }
        Commands::InstallIn { lib } => {
            cyber_print(&format!("Installing {} in project...", lib));
            deps::install_in(&lib);
        }
        Commands::InstallO { lib } => {
            cyber_print(&format!("Installing {} in isolated env...", lib));
            deps::install_o(&lib);
        }
        Commands::Add { pkg } => {
            cyber_print(&format!("Adding package {}...", pkg));
            deps::add_package(&pkg);
        }
        Commands::Build { deb, rpm, bin, appimage } => {
            cyber_print("Building project...");
            let options = vec![deb, rpm, bin, appimage]
                .into_iter()
                .zip(["deb", "rpm", "bin", "appimage"])
                .filter_map(|(flag, opt)| if flag { Some(opt) } else { None })
                .collect::<Vec<_>>();
            build::build_project(&options);
        }
        Commands::Run => {
            cyber_print("Running in dev mode...");
            run::run_dev();
        }
        Commands::Test => {
            cyber_print("Running tests...");
            build::run_tests();
        }
        Commands::Fmt => {
            cyber_print("Formatting code...");
            // Call velvet-fmt
            std::process::Command::new("cargo")
                .args(&["run", "--bin", "velvet-fmt"])
                .status()
                .unwrap();
        }
        Commands::Doc => {
            cyber_print("Generating documentation...");
            // Placeholder: Generate HTML docs
        }
        Commands::Repl => {
            cyber_print("Starting Velvet REPL...");
            std::process::Command::new("cargo")
                .args(&["run", "--bin", "velvet-repl"])
                .status()
                .unwrap();
        }
    }
}
