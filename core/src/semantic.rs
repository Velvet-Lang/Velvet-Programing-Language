use std::collections::{HashMap, HashSet};
use crate::parser::AstNode;

#[derive(Debug, Clone)]
enum Type {
    Int,
    Float,
    String,
    Bool,
    Result(Box<Type>, Box<Type>),
    Option(Box<Type>),
    Custom(String),
}

pub fn analyze_ast(ast: &[AstNode]) -> Result<(), String> {
    let mut variables: HashMap<String, (Type, bool)> = HashMap::new(); // (type, is_mutable)
    let mut types: HashSet<String> = HashSet::new();
    let mut traits: HashMap<String, Vec<(String, String)>> = HashMap::new();

    for node in ast {
        match node {
            AstNode::Dependency(dep) => {
                if dep.is_empty() {
                    return Err("Empty dependency name".to_string());
                }
            }
            AstNode::Embed(lang, _) => {
                if !["python", "js"].contains(&lang.as_str()) {
                    return Err(format!("Unsupported embed language: {}", lang));
                }
            }
            AstNode::Let { mutable, name, ty, value } => {
                let inferred_ty = match value.parse::<i32>() {
                    Ok(_) => Type::Int,
                    Err(_) => match value.as_str() {
                        "true" | "false" => Type::Bool,
                        _ if value.starts_with('"') => Type::String,
                        _ => Type::Custom(value.to_string()), // Placeholder for complex expr
                    },
                };
                let var_ty = ty.as_ref().map(|t| match t.as_str() {
                    "int" => Type::Int,
                    "string" => Type::String,
                    "bool" => Type::Bool,
                    _ => Type::Custom(t.to_string()),
                }).unwrap_or(inferred_ty);
                variables.insert(name.clone(), (var_ty, *mutable));
            }
            AstNode::If(expr, _) => {
                let parts: Vec<&str> = expr.split("==").collect();
                if parts.len() == 2 && !variables.contains_key(parts[0].trim()) {
                    return Err(format!("Undefined variable in if: {}", parts[0]));
                }
            }
            AstNode::Loop(count, _) => {
                if count.parse::<u32>().is_err() && !variables.contains_key(count) {
                    return Err(format!("Invalid loop count: {}", count));
                }
            }
            AstNode::Match { value, arms } => {
                if !variables.contains_key(value) {
                    return Err(format!("Undefined variable in match: {}", value));
                }
                for (pattern, range_end, _) in arms {
                    if let Some(end) = range_end {
                        if !end.parse::<i32>().is_ok() && !variables.contains_key(end) {
                            return Err(format!("Invalid range end in match: {}", end));
                        }
                    }
                }
            }
            AstNode::TypeDef { name, expr } => {
                types.insert(name.clone());
                // Validate expr (stub)
            }
            AstNode::TraitDef { name, methods } => {
                traits.insert(name.clone(), methods.clone());
            }
            AstNode::Spawn(expr) => {
                // Validate async-compatible expr (stub)
            }
            AstNode::Try(expr) => {
                if !variables.contains_key(expr) || !matches!(variables[expr].0, Type::Result(_, _)) {
                    return Err(format!("Try used on non-Result type: {}", expr));
                }
            }
            AstNode::Output(expr) => {
                if expr.contains("$INDEX") {
                    let in_loop = ast.iter().any(|n| matches!(n, AstNode::Loop(_, _)));
                    if !in_loop {
                        return Err("$INDEX used outside loop".to_string());
                    }
                }
            }
            _ => {}
        }
    }

    // Lightweight borrow checker: Ensure mutable variables aren't shared unsafely
    for node in ast {
        if let AstNode::Spawn(expr) = node {
            if let Some(var) = variables.get(expr) {
                if var.1 {
                    return Err(format!("Mutable variable {} used in spawn", expr));
                }
            }
        }
    }

    Ok(())
}
