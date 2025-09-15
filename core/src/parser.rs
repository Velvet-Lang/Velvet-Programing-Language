// velvet-core/src/parser.rs
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use anyhow::Result;

use crate::VelvetAst;

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
                                    _ => {
                                        if part.as_str() == "then" {
                                            in_then = true;
                                        }
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

// Helper do rekurencyjnego parsowania statements (np. w if body)
fn parse_statement(pair: &Pair<Rule>) -> Result<Vec<VelvetAst>> {
    let mut sub_ast = Vec::new();
    for inner in pair.clone().into_inner() {
        match inner.as_rule() {
            Rule::dependency => {
                let dep = inner.into_inner().next().unwrap().as_str().to_string();
                sub_ast.push(VelvetAst::Dependency(dep));
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
                sub_ast.push(VelvetAst::Command(cmd, args));
                if let Some(r) = redir {
                    sub_ast.push(r);
                }
            }
            Rule::COMMENT => {
                let comment = inner.as_str().trim_start_matches('@').trim().to_string();
                sub_ast.push(VelvetAst::Comment(comment));
            }
            Rule::if_block => {
                // Rekurencja dla nested if
                let sub_if = parse_velvet(inner.as_str())?;
                sub_ast.extend(sub_if);
            }
            _ => {}
        }
    }
    Ok(sub_ast)
                                 }
