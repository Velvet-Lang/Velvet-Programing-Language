use anyhow::Result;
use crate::VelvetAst;
use std::process::Command;

pub fn execute_ast(ast: &[VelvetAst]) -> Result<()> {
    for node in ast {
        match node {
            VelvetAst::Command(cmd, args) => {
                let output = Command::new(cmd).args(args).output()?;
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            VelvetAst::IfThen(cond, body) => {
                // Uproszczone: zakładaj true
                execute_ast(body)?;
            }
            _ => {}  // Ignoruj deps/comments/redir (dla uproszczenia)
        }
    }
    Ok(())
}
