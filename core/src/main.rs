mod parser;
mod semantic;
mod codegen;
mod optimizer;

use std::env;
use std::fs;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 { return Err("Usage: velvetc input.vel -o output.o".to_string()); }

    let input = fs::read_to_string(&args[1]).map_err(|e| e.to_string())?;
    let (_, ast) = parser::parse_velvet(&input).map_err(|e| e.to_string())?;
    let optimized = optimizer::optimize_ast(ast);
    semantic::analyze_ast(&optimized)?;
    codegen::generate_o(&optimized, &args[3])?;
    Ok(())
}
