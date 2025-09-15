// velvet-core/src/lib.rs
pub mod parser;
pub mod compiler;
pub mod ffi;
pub mod runtime;

use anyhow::Result;
use nom::IResult;

// Przykładowa struktura AST dla Velvet (shell-like)
#[derive(Debug)]
pub enum VelvetAst {
    Command(String, Vec<String>),  // e.g., "echo" "Hello"
    Dependency(String),            // [dep]
    Comment(String),
    IoRedir(String, String),       // > file
    IfThen(Vec<VelvetAst>, Vec<VelvetAst>),
}

// Parser (uproszczony nom-based dla shell składni)
pub fn parse_velvet(input: &str) -> Result<Vec<VelvetAst>> {
    let (_, ast) = parser::velvet_parser(input).map_err(|e| anyhow::anyhow!("Parse error: {:?}", e))?;
    Ok(ast)
}

// Kompilator: translacja do Rust code (dla bezpieczeństwa i wydajności)
pub fn compile_to_rust(ast: &[VelvetAst], output: &str) -> Result<()> {
    let rust_code = compiler::generate_rust(ast);
    std::fs::write(output, rust_code)?;
    Ok(())
}

// Runtime: Uruchom kompilowany kod (bezpieczeństwo jak Rust)
pub fn run_compiled(rust_path: &str) -> Result<()> {
    // Użyj std::process::Command do kompilacji i run (w realu: invoke cargo run)
    let output = std::process::Command::new("cargo")
        .arg("run")
        .arg("--manifest-path")
        .arg(rust_path)
        .output()?;
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}
