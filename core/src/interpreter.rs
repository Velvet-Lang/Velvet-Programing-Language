use std::collections::HashMap;
use anyhow::Result;

pub fn run_decl_program(program: &str) -> Result<()> {
    let mut memory: HashMap<String, i64> = HashMap::new();
    let globals: HashMap<String, fn(i64) -> i64> = [
        ("inc".to_string(), |x: i64| x + 1),
        ("square".to_string(), |x: i64| x * x),
    ].iter().cloned().collect();

    for line in program.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        if line.starts_with("?") {
            let var = line[1..].trim().to_string();
            println!("{}", memory.get(&var).copied().unwrap_or(0));
            continue;
        }
        if line.contains("<") {
            let parts: Vec<&str> = line.split("<").map(|s| s.trim()).collect();
            let val = parts[0].parse::<i64>().unwrap();
            let var = parts[1].to_string();
            memory.insert(var, val);
            continue;
        }
        if line.contains(">") {
            let parts: Vec<&str> = line.split(">").map(|s| s.trim()).collect();
            let left = parts[0];
            let right = parts[1].to_string();
            let mut val: i64 = 0;
            if left.contains("*") {
                let mul: Vec<&str> = left.split("*").map(|s| s.trim()).collect();
                let a = memory.get(mul[0]).copied().unwrap_or(0);
                let b = memory.get(mul[1]).copied().unwrap_or(0);
                val = a * b;
            } else if left.contains("^") {
                let pipe: Vec<&str> = left.split("^").map(|s| s.trim()).collect();
                val = memory.get(pipe[0]).copied().unwrap_or(0);
                for func_name in &pipe[1..] {
                    if let Some(f) = globals.get(*func_name) {
                        val = f(val);
                    } else {
                        // Error or skip
                    }
                }
            } else {
                val = memory.get(left).copied().unwrap_or(0);
            }
            memory.insert(right, val);
        }
    }
    Ok(())
}
