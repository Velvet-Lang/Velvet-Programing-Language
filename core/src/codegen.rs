use std::fs::File;
use std::io::Write;
use std::process::Command;
use crate::parser::AstNode;

pub fn generate_o(ast: &[AstNode], output: &str) -> Result<(), String> {
    let mut asm = String::new();
    asm.push_str(".global _start\n_start:\n");

    for node in ast {
        match node {
            AstNode::Output(s) => {
                asm.push_str(&format!("mov $1, %rax\nmov ${}, %rdi\nsyscall\n", s.len()));  // Stub sys_write
                // Actual: Generate code for print
            }
            AstNode::Embed("python", code) => {
                // Embed: Generate FFI call to Python runtime (stub)
            }
            _ => {},
        }
    }

    let asm_file = "temp.s";
    let mut file = File::create(asm_file).map_err(|e| e.to_string())?;
    file.write_all(asm.as_bytes()).map_err(|e| e.to_string())?;

    // Assemble to .o (using system assembler, no LLVM)
    Command::new("as").arg(asm_file).arg("-o").arg(output).output().map_err(|e| e.to_string())?;
    Ok(())
}
