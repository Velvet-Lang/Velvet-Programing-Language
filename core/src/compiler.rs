use crate::VelvetAst;

pub fn generate_rust(ast: &[VelvetAst], deps: &[String]) -> String {
    let mut code = String::new();
    code.push_str("use std::process::Command;\n");
    for dep in deps {
        code.push_str(&format!("extern crate {};\n", dep));  // Zakładamy deps to crates
    }
    code.push_str("fn main() {\n");
    for node in ast {
        match node {
            VelvetAst::Command(cmd, args) => {
                code.push_str(&format!("    Command::new(\"{}\").args([{}]).output().unwrap();\n", cmd, args.iter().map(|a| format!("\"{}\"", a)).collect::<Vec<_>>().join(", ")));
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
                code.push_str("    if true {  // TODO: real cond\n");
                code.push_str(&generate_rust(body, deps));
                code.push_str("    }\n");
            }
        }
    }
    code.push_str("}\n");
    code
}
