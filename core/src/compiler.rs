use crate::{VelvetAst, DeclAst};

pub fn generate_rust(ast: &[VelvetAst], deps: &[String]) -> String {
    let mut code = String::new();
    code.push_str("use std::process::Command;\nuse std::collections::HashMap;\nuse pyo3::prelude::*;\n");
    for dep in deps {
        code.push_str(&format!("extern crate {};\n", dep));
    }
    code.push_str("fn main() {\n    let mut memory: HashMap<String, i64> = HashMap::new();\n");
    for node in ast {
        match node {
            VelvetAst::Section(lang, inner) => {
                if lang == "python" {
                    code.push_str("    Python::with_gil(|py| py.run_bound(r#\"");
                    code.push_str(inner);
                    code.push_str("\"#, None, None).unwrap());\n");
                } else if lang == "velvet" {
                    code.push_str(&generate_decl(inner));
                }
            }
            VelvetAst::SysCommand(cmd) => {
                code.push_str(&format!("    Command::new(\"sh\").arg(\"-c\").arg(\"{}\").output().unwrap();\n", cmd));
            }
            VelvetAst::DeclLine(decl) => {
                match decl {
                    DeclAst::Assign(val, var) => {
                        code.push_str(&format!("    memory.insert(\"{}\".to_string(), {}.parse().unwrap());\n", var, val));
                    }
                    DeclAst::Transform(a, op, b) => {
                        if op == "*" {
                            code.push_str(&format!("    let val = memory[\"{}\"] * memory[\"{}\"];\n", a, b));
                            // Zakładaj insert do var from AST context
                        }
                    }
                    DeclAst::Pipeline(parts) => {
                        let mut val = format!("memory.get(\"{}\").cloned().unwrap_or(0)", parts[0]);
                        for func in &parts[1..] {
                            val = format!("{}( {})", func, val);
                        }
                        code.push_str(&format!("    let val = {};\n", val));
                    }
                    DeclAst::Query(var) => {
                        code.push_str(&format!("    println!(\"{{}}\", memory[\"{}\"]);\n", var));
                    }
                }
            }
            VelvetAst::Command(cmd, args) => {
                code.push_str(&format!("    Command::new(\"{}\").args(&[{}]).output().unwrap();\n", cmd, args.iter().map(|a| format!("\"{}\"", a)).join(", ")));
            }
            VelvetAst::Dependency(dep) => {
                code.push_str(&format!("    // Dep: {}\n", dep));
            }
            VelvetAst::Comment(c) => {
                code.push_str(&format!("    // {}\n", c));
            }
            VelvetAst::IoRedir(dir, file) => {
                code.push_str(&format!("    // Redirect {} to {}\n", dir, file));
            }
            VelvetAst::IfThen(cond, body) => {
                code.push_str("    if true { // TODO cond\n");
                code.push_str(&generate_rust(body, deps));
                code.push_str("    }\n");
            }
        }
    }
    code.push_str("}\nfn inc(x: i64) -> i64 { x + 1 }\nfn square(x: i64) -> i64 { x * x }\n");
    code
}

fn generate_decl(inner: &str) -> String {
    let mut code = String::new();
    code.push_str("    // Declarative Velvet\n");
    // Użyj velvet::interpreter::run_decl_program(inner).unwrap(); ale dla compile - inline
    code
}
