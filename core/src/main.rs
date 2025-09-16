use std::env;
use std::fs;

mod parser;
mod semantic;
mod codegen;
mod optimizer;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 || args[2] != "-o" {
        return Err("Usage: velvetc input.vel -o output.o".to_string());
    }

    let input_file = &args[1];
    let output_file = &args[3];

    let input = fs::read_to_string(input_file).map_err(|e| format!("Failed to read {}: {}", input_file, e))?;
    let ast = parser::parse_velvet(&input)?;
    let optimized = optimizer::optimize_ast(ast);
    semantic::analyze_ast(&optimized)?;
    codegen::generate_o(&optimized, output_file)?;
    println!("Compiled {} to {}", input_file, output_file);
    Ok(())
}
