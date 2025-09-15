// velvet-core/src/compiler.rs
use crate::VelvetAst;

pub fn generate_rust(ast: &[VelvetAst]) -> String {
    let mut code = String::new();
    code.push_str("fn main() {\n");
    for node in ast {
        match node {
            VelvetAst::Command(cmd, args) => {
                // Translacja do safe Rust (np. Command z std::process, borrow checked)
                code.push_str(&format!("    std::process::Command::new(\"{}\").args([{}]).output().unwrap();\n", cmd, args.join(", ")));
            }
            VelvetAst::Dependency(dep) => {
                code.push_str(&format!("    // Imported: {}\n", dep));
            }
            VelvetAst::Comment(c) => {
                code.push_str(&format!("    // {}\n", c));
            }
            VelvetAst::IoRedir(dir, file) => {
                code.push_str(&format!("    // Redirect {} to {}\n", dir, file));
            }
            _ => {}
        }
    }
    code.push_str("}\n");
    code
}
