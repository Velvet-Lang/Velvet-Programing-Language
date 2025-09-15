// velvet-core/src/lib.rs
pub mod parser;
pub mod compiler;
pub mod ffi;
pub mod runtime;

use anyhow::Result;
use std::fs;
use std::path::Path;

// Zaktualizowany AST: dodano sekcje
#[derive(Debug)]
pub enum VelvetAst {
    Command(String, Vec<String>),
    Dependency(String),
    Comment(String),
    IoRedir(String, String),
    IfThen(Vec<VelvetAst>, Vec<VelvetAst>),
    Section(String, String),  // lang, inner_code (np. "python", "kod")
    SysCommand(String),      // OS-specific cmd (detect in parser)
    DeclLine(DeclAst),       // Dla #velvet: sub-AST declarative
}

#[derive(Debug)]
pub enum DeclAst {
    Assign(String, String),  // val < var
    Transform(String, String, String),  // left > right, with op
    Pipeline(Vec<String>),   // expr ^ func ^ ...
    Query(String),           // ?var
}

// Parse
pub fn parse_velvet(input: &str) -> Result<Vec<VelvetAst>> {
    parser::parse_velvet(input)
}

// Zbieraj deps (bez zmian)
pub fn collect_deps(ast: &[VelvetAst]) -> Vec<String> {
    let mut deps = Vec::new();
    for node in ast {
        if let VelvetAst::Dependency(dep) = node {
            deps.push(dep.clone());
        }
        if let VelvetAst::Section(_, inner) = node {
            // Rekurencyjnie parse inner jeśli #velvet
            if let Ok(sub_ast) = parse_velvet(inner) {
                deps.extend(collect_deps(&sub_ast));
            }
        }
        // ...
    }
    deps
}

// Skanuj projekt (bez zmian, ale dodaj check OS-specific)
pub fn scan_project(project_dir: &str) -> Result<(Vec<String>, Vec<String>)> {
    // ...
}

// Kompilator
pub fn compile_to_rust(ast: &[VelvetAst], output: &str, deps: &[String]) -> Result<()> {
    let rust_code = compiler::generate_rust(ast, deps);
    fs::write(output, rust_code)?;
    Ok(())
}

// Runtime: exec AST, w tym sections
pub fn run_velvet(input: &str) -> Result<()> {
    let ast = parse_velvet(input)?;
    runtime::execute_ast(&ast)?;
    Ok(())
}
