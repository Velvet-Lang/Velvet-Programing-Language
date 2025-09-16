use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../velvet.pest"]
pub struct VelvetParser;

#[derive(Debug, Clone)]
pub enum AstNode {
    Dependency(String),
    Comment(String),
    Embed(String, String), // Language, Code
    Output(String),
    If(String, Box<AstNode>),
    Loop(String, Box<AstNode>),
}

pub fn parse_velvet(input: &str) -> Result<Vec<AstNode>, String> {
    let pairs = VelvetParser::parse(Rule::program, input)
        .map_err(|e| format!("Parse error: {}", e))?;
    let mut ast = Vec::new();

    for pair in pairs {
        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::dependency => {
                    let ident = inner.into_inner().next().unwrap().as_str().to_string();
                    ast.push(AstNode::Dependency(ident));
                }
                Rule::COMMENT => {
                    let comment = inner.as_str().trim_start_matches('@').trim().to_string();
                    ast.push(AstNode::Comment(comment));
                }
                Rule::embed => {
                    let mut inner = inner.into_inner();
                    let lang = inner.next().unwrap().as_str().to_string();
                    let code = inner.next().unwrap().as_str().to_string();
                    ast.push(AstNode::Embed(lang, code));
                }
                Rule::output => {
                    let string = inner.into_inner().next().unwrap().as_str().trim_matches('"').to_string();
                    ast.push(AstNode::Output(string));
                }
                Rule::if_stmt => {
                    let mut inner = inner.into_inner();
                    let expr = inner.next().unwrap().as_str().to_string();
                    let cmd = inner.next().unwrap();
                    let cmd_node = match cmd.as_rule() {
                        Rule::output => {
                            let s = cmd.into_inner().next().unwrap().as_str().trim_matches('"').to_string();
                            AstNode::Output(s)
                        }
                        _ => unreachable!(),
                    };
                    ast.push(AstNode::If(expr, Box::new(cmd_node)));
                }
                Rule::loop_stmt => {
                    let mut inner = inner.into_inner();
                    let count = inner.next().unwrap().as_str().to_string();
                    let cmd = inner.next().unwrap();
                    let cmd_node = match cmd.as_rule() {
                        Rule::output => {
                            let s = cmd.into_inner().next().unwrap().as_str().trim_matches('"').to_string();
                            AstNode::Output(s)
                        }
                        _ => unreachable!(),
                    };
                    ast.push(AstNode::Loop(count, Box::new(cmd_node)));
                }
                _ => {}
            }
        }
    }
    Ok(ast)
}
