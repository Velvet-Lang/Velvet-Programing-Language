use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use anyhow::Result;

use crate::{VelvetAst, DeclAst};

#[derive(Parser)]
#[grammar = "velvet.pest"]
pub struct VelvetParser;

pub fn parse_velvet(input: &str) -> Result<Vec<VelvetAst>> {
    let pairs = VelvetParser::parse(Rule::program, input)
        .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;
    
    let mut ast = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::statement => {
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::dependency => {
                            let dep = inner.into_inner().next().unwrap().as_str().to_string();
                            ast.push(VelvetAst::Dependency(dep));
                        }
                        Rule::command => {
                            let mut cmd = String::new();
                            let mut args = Vec::new();
                            let mut redir = None;
                            for part in inner.into_inner() {
                                match part.as_rule() {
                                    Rule::ident => {
                                        if cmd.is_empty() {
                                            cmd = part.as_str().to_string();
                                        } else {
                                            args.push(part.as_str().to_string());
                                        }
                                    }
                                    Rule::string => args.push(part.as_str().trim_matches('"').to_string()),
                                    Rule::redir => {
                                        let mut r = part.into_inner();
                                        let dir = r.next().unwrap().as_str().to_string();
                                        let file = r.next().unwrap().as_str().to_string();
                                        redir = Some(VelvetAst::IoRedir(dir, file));
                                    }
                                    _ => {}
                                }
                            }
                            ast.push(VelvetAst::Command(cmd, args));
                            if let Some(r) = redir {
                                ast.push(r);
                            }
                        }
                        Rule::COMMENT => {
                            let comment = inner.as_str().trim_start_matches('@').trim().to_string();
                            ast.push(VelvetAst::Comment(comment));
                        }
                        Rule::section => {
                            let mut parts = inner.into_inner();
                            let lang = parts.next().unwrap().as_str().to_string();
                            let code = parts.map(|p| p.as_str()).collect::<Vec<_>>().join("");
                            ast.push(VelvetAst::Section(lang, code));
                        }
                        Rule::sys_command => {
                            let cmd = inner.as_str().to_string();
                            ast.push(VelvetAst::SysCommand(cmd));
                        }
                        Rule::decl_line => {
                            let str_line = inner.as_str();
                            if str_line.contains("<") {
                                let parts: Vec<&str> = str_line.split("<").map(|s| s.trim()).collect();
                                ast.push(VelvetAst::DeclLine(DeclAst::Assign(parts[0].to_string(), parts[1].to_string())));
                            } else if str_line.contains(">") {
                                let parts: Vec<&str> = str_line.split(">").map(|s| s.trim()).collect();
                                let left = parts[0];
                                let right = parts[1].to_string();
                                if left.contains("^") {
                                    let pipe: Vec<String> = left.split("^").map(|s| s.trim().to_string()).collect();
                                    ast.push(VelvetAst::DeclLine(DeclAst::Pipeline(pipe)));
                                    // Zakładamy > right jako separate assign
                                } else if left.contains("*") {
                                    let mul: Vec<&str> = left.split("*").map(|s| s.trim()).collect();
                                    ast.push(VelvetAst::DeclLine(DeclAst::Transform(mul[0].to_string(), "*".to_string(), mul[1].to_string())));
                                }
                            } else if str_line.starts_with("?") {
                                ast.push(VelvetAst::DeclLine(DeclAst::Query(str_line[1..].trim().to_string())));
                            }
                        }
                        Rule::if_block => {
                            let mut condition = Vec::new();
                            let mut body = Vec::new();
                            let mut in_then = false;
                            for part in inner.into_inner() {
                                match part.as_rule() {
                                    Rule::ident => {
                                        if !in_then {
                                            condition.push(VelvetAst::Command(part.as_str().to_string(), vec![]));
                                        }
                                    }
                                    Rule::statement => {
                                        if in_then {
                                            let sub_ast = parse_statement(&part)?;
                                            body.extend(sub_ast);
                                        }
                                    }
                                    _ => if part.as_str() == "then" {
                                        in_then = true;
                                    }
                                }
                            }
                            ast.push(VelvetAst::IfThen(condition, body));
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    Ok(ast)
}

fn parse_statement(pair: &Pair<Rule>) -> Result<Vec<VelvetAst>> {
    // Podobnie jak w main parse, rekurencyjnie
    let mut sub_ast = Vec::new();
    // ... implementacja podobna do powyżej
    Ok(sub_ast)
}
