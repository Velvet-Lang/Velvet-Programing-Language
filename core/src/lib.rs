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
    Transform(String, String, String),  // left op right, np. A * B
    Pipeline(Vec<String>),   // expr ^ func ^ ...
    Query(String),           // ?var
}

// Parse
pub fn parse_velvet(input: &str) -> Result<Vec<VelvetAst>> {
    parser::parse_velvet(input)
}

// Zbieraj deps
pub fn collect_deps(ast: &[VelvetAst]) -> Vec<String> {
    let mut deps = Vec::new();
    for node in ast {
        if let VelvetAst::Dependency(dep) = node {
            deps.push(dep.clone());
        }
        if let VelvetAst::Section(_, inner) = node {
            if let Ok(sub_ast) = parse_velvet(inner) {
                deps.extend(collect_deps(&sub_ast));
            }
        }
        if let VelvetAst::IfThen(_, body) = node {
            deps.extend(collect_deps(body));
        }
    }
    deps
}

// Skanuj projekt
pub fn scan_project(project_dir: &str) -> Result<(Vec<String>, Vec<String>)> {
    let mut all_deps = Vec::new();
    let mut errors = Vec::new();
    for entry in fs::read_dir(project_dir)? {
        let path = entry?.path();
        if path.is_file() && path.extension().map_or(false, |e| e == "vel") {
            let content = fs::read_to_string(&path)?;
            match parse_velvet(&content) {
                Ok(ast) => all_deps.extend(collect_deps(&ast)),
                Err(e) => errors.push(format!("Error in {:?}: {}", path, e)),
            }
        } else if path.is_dir() {
            let (sub_deps, sub_errs) = scan_project(path.to_str().unwrap())?;
            all_deps.extend(sub_deps);
            errors.extend(sub_errs);
        }
    }
    Ok((all_deps, errors))
}

// Kompilator
pub fn compile_to_rust(ast: &[VelvetAst], output: &str, deps: &[String]) -> Result<()> {
    let rust_code = compiler::generate_rust(ast, deps);
    fs::write(output, rust_code)?;
    Ok(())
}

// Runtime
pub fn run_velvet(input: &str) -> Result<()> {
    let ast = parse_velvet(input)?;
    runtime::execute_ast(&ast)?;
    Ok(())
}
