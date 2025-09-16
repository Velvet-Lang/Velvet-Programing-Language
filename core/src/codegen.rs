use std::fs::File;
use std::io::Write;
use std::process::Command;
use crate::parser::AstNode;

pub fn generate_o(ast: &[AstNode], output: &str) -> Result<(), String> {
    let mut asm = String::new();
    asm.push_str(".global _start\n_start:\n");
    let mut label_count = 0;

    for node in ast {
        match node {
            AstNode::Output(s) => {
                let msg_label = format!("msg_{}", label_count);
                asm.push_str(&format!(
                    "mov $1, %rax\nmov $1, %rdi\nlea {}, %rsi\nmov ${}, %rdx\nsyscall\n",
                    msg_label, s.len()
                ));
                asm.push_str(&format!("{}: .ascii \"{}\"\n", msg_label, s));
                label_count += 1;
            }
            AstNode::Embed("python", code) => {
                asm.push_str("; Python FFI (stub)\n");
                // Placeholder: Call Python runtime via libpython
            }
            AstNode::Embed("shell", code) => {
                asm.push_str("; Shell FFI (stub)\n");
                // Placeholder: Call system()
            }
            AstNode::If(expr, cmd) => {
                let parts: Vec<&str> = expr.split("==").collect();
                if parts.len() == 2 {
                    let label_else = format!("else_{}", label_count);
                    let label_end = format!("end_{}", label_count);
                    asm.push_str(&format!("cmp ${}, %rax\njne {}\n", parts[1].trim(), label_else));
                    if let AstNode::Output(s) = cmd.as_ref() {
                        let msg_label = format!("msg_{}", label_count);
                        asm.push_str(&format!(
                            "mov $1, %rax\nmov $1, %rdi\nlea {}, %rsi\nmov ${}, %rdx\nsyscall\n",
                            msg_label, s.len()
                        ));
                        asm.push_str(&format!("{}: .ascii \"{}\"\n", msg_label, s));
                    }
                    asm.push_str(&format!("jmp {}\n{}:\n", label_end, label_else));
                    asm.push_str(&format!("{}:\n", label_end));
                    label_count += 1;
                }
            }
            AstNode::Loop(count, cmd) => {
                if let Ok(n) = count.parse::<u32>() {
                    let loop_start = format!("loop_{}", label_count);
                    let loop_end = format!("loop_end_{}", label_count);
                    asm.push_str(&format!("mov ${}, %rcx\n", n));
                    asm.push_str(&format!("{}:\n", loop_start));
                    if let AstNode::Output(s) = cmd.as_ref() {
                        let msg_label = format!("msg_{}", label_count);
                        asm.push_str(&format!(
                            "mov $1, %rax\nmov $1, %rdi\nlea {}, %rsi\nmov ${}, %rdx\nsyscall\n",
                            msg_label, s.len()
                        ));
                        asm.push_str(&format!("{}: .ascii \"{}\"\n", msg_label, s));
                    }
                    asm.push_str("dec %rcx\n");
                    asm.push_str(&format!("jnz {}\n", loop_start));
                    asm.push_str(&format!("{}:\n", loop_end));
                    label_count += 1;
                }
            }
            _ => {}
        }
    }

    asm.push_str("mov $60, %rax\nxor %rdi, %rdi\nsyscall\n");

    fs::create_dir_all("build").map_err(|e| e.to_string())?;
    let asm_file = "build/temp.s";
    let mut file = File::create(asm_file).map_err(|e| format!("Failed to create {}: {}", asm_file, e))?;
    file.write_all(asm.as_bytes()).map_err(|e| e.to_string())?;

    let output = Command::new("as")
        .arg(asm_file)
        .arg("-o")
        .arg(output)
        .output()
        .map_err(|e| format!("Assembly failed: {}", e))?;
    if !output.status.success() {
        return Err(format!("Assembly error: {}", String::from_utf8_lossy(&output.stderr)));
    }
    Ok(())
}
