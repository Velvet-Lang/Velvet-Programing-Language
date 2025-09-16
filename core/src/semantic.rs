use crate::parser::AstNode;

pub fn analyze_ast(ast: &[AstNode]) -> Result<(), String> {
    let mut variables = std::collections::HashSet::new();
    for node in ast {
        match node {
            AstNode::Dependency(dep) => {
                if dep.is_empty() {
                    return Err("Empty dependency name".to_string());
                }
            }
            AstNode::Embed(lang, _) => {
                if !["python", "shell"].contains(&lang.as_str()) {
                    return Err(format!("Unsupported embed language: {}", lang));
                }
            }
            AstNode::If(expr, _) => {
                // Simple check: Ensure expr references known variables
                let parts: Vec<&str> = expr.split("==").collect();
                if parts.len() == 2 && !variables.contains(parts[0].trim()) {
                    return Err(format!("Undefined variable in if: {}", parts[0]));
                }
            }
            AstNode::Loop(count, _) => {
                if count.parse::<u32>().is_err() && !variables.contains(count) {
                    return Err(format!("Invalid loop count: {}", count));
                }
            }
            AstNode::Output(s) => {
                if s.contains("$INDEX") {
                    // Ensure $INDEX is used in a loop
                    let in_loop = ast.iter().any(|n| matches!(n, AstNode::Loop(_, _)));
                    if !in_loop {
                        return Err("$INDEX used outside loop".to_string());
                    }
                }
            }
            _ => {}
        }
        // Track variables from embeds (stub)
        if let AstNode::Embed("python", code) = node {
            if code.contains("result =") {
                variables.insert("result".to_string());
            }
        }
    }
    Ok(())
}
