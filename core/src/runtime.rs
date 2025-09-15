use anyhow::Result;
use crate::{VelvetAst, DeclAst};
use std::process::Command;
use std::collections::HashMap;
use pyo3::prelude::*;
use crate::velvet::interpreter::run_decl_program;

pub fn execute_ast(ast: &[VelvetAst]) -> Result<()> {
    let mut memory: HashMap<String, i64> = HashMap::new();
    let globals: HashMap<String, fn(i64) -> i64> = [
        ("inc".to_string(), inc as fn(i64) -> i64),
        ("square".to_string(), square as fn(i64) -> i64),
    ].iter().cloned().collect();

    for node in ast {
        match node {
            VelvetAst::Section(lang, inner) => {
                if lang == "python" {
                    Python::with_gil(|py| {
                        py.run_bound(inner.as_str(), None, None)?;
                        Ok(())
                    })?;
                } else if lang == "velvet" {
                    run_decl_program(inner.as_str())?;
                } else {
                    // Dla #inne - skip lub error
                }
            }
            VelvetAst::SysCommand(cmd) => {
                let shell = if cfg!(target_os = "windows") { "powershell" } else { "sh" };
                Command::new(shell).arg("-c").arg(cmd).output()?;
            }
            VelvetAst::DeclLine(decl) => {
                match decl {
                    DeclAst::Assign(val, var) => {
                        memory.insert(var.clone(), val.parse::<i64>().unwrap());
                    }
                    DeclAst::Transform(a, op, b) => {
                        if op == "*" {
                            let val_a = *memory.get(a).unwrap_or(&0);
                            let val_b = *memory.get(b).unwrap_or(&0);
                            // Insert to temp or context var
                        }
                    }
                    DeclAst::Pipeline(parts) => {
                        let mut val = *memory.get(&parts[0]).unwrap_or(&0);
                        for func in &parts[1..] {
                            if let Some(f) = globals.get(func) {
                                val = f(val);
                            }
                        }
                    }
                    DeclAst::Query(var) => {
                        println!("{}", memory.get(var).unwrap_or(&0));
                    }
                }
            }
            _ => {}  // Inne nodes
        }
    }
    Ok(())
}

fn inc(x: i64) -> i64 { x + 1 }
fn square(x: i64) -> i64 { x * x }
