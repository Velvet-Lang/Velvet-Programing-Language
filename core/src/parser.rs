use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../velvet.pest"]
pub struct VelvetParser;

#[derive(Debug, Clone)]
pub enum AstNode {
    Dependency(String),
    Comment(String),
    Embed(String, String),
    Output(String),
    If(String, Box<AstNode>),
    Loop(String, Box<AstNode>),
    Let { mutable: bool, name: String, ty: Option<String>, value: String },
    Match { value: String, arms: Vec<(String, Option<String>, String)> },
    Spawn(String),
    Try(String),
    TypeDef { name: String, expr: String },
    TraitDef { name: String, methods: Vec<(String, String)> },
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
                    let expr = inner.into_inner().next().unwrap().as_str().to_string();
                    ast.push(AstNode::Output(expr));
                }
                Rule::if_stmt => {
                    let mut inner = inner.into_inner();
                    let expr = inner.next().unwrap().as_str().to_string();
                    let cmd = parse_command(inner.next().unwrap())?;
                    ast.push(AstNode::If(expr, Box::new(cmd)));
                }
                Rule::loop_stmt => {
                    let mut inner = inner.into_inner();
                    let count = inner.next().unwrap().as_str().to_string();
                    let cmd = parse_command(inner.next().unwrap())?;
                    ast.push(AstNode::Loop(count, Box::new(cmd)));
                }
                Rule::let_stmt => {
                    let mut inner = inner.into_inner();
                    let mutable = inner.next().map(|p| p.as_str() == "mut").unwrap_or(false);
                    let name = inner.next().unwrap().as_str().to_string();
                    let ty = inner.next().and_then(|p| p.into_inner().next()).map(|p| p.as_str().to_string());
                    let value = inner.next().unwrap().as_str().to_string();
                    ast.push(AstNode::Let { mutable, name, ty, value });
                }
                Rule::match_stmt => {
                    let mut inner = inner.into_inner();
                    let value = inner.next().unwrap().as_str().to_string();
                    let mut arms = Vec::new();
                    for arm in inner {
                        let mut arm_inner = arm.into_inner();
                        let pattern = arm_inner.next().unwrap().as_str().to_string();
                        let range_end = arm_inner.next().map(|p| p.as_str().to_string());
                        let expr = arm_inner.next().unwrap().as_str().to_string();
                        arms.push((pattern, range_end, expr));
                    }
                    ast.push(AstNode::Match { value, arms });
                }
                Rule::spawn_stmt => {
                    let expr = inner.into_inner().next().unwrap().as_str().to_string();
                    ast.push(AstNode::Spawn(expr));
                }
                Rule::try_stmt => {
                    let expr = inner.into_inner().next().unwrap().as_str().to_string();
                    ast.push(AstNode::Try(expr));
                }
                Rule::type_def => {
                    let mut inner = inner.into_inner();
                    let name = inner.next().unwrap().as_str().to_string();
                    let expr = inner.next().unwrap().as_str().to_string();
                    ast.push(AstNode::TypeDef { name, expr });
                }
                Rule::trait_def => {
                    let mut inner = inner.into_inner();
                    let name = inner.next().unwrap().as_str().to_string();
                    let mut methods = Vec::new();
                    for method in inner {
                        let mut method_inner = method.into_inner();
                        let method_name = method_inner.next().unwrap().as_str().to_string();
                        let method_ty = method_inner.next().unwrap().as_str().to_string();
                        methods.push((method_name, method_ty));
                    }
                    ast.push(AstNode::TraitDef { name, methods });
                }
                _ => {}
            }
        }
    }
    Ok(ast)
}

fn parse_command(pair: pest::iterators::Pair<Rule>) -> Result<AstNode, String> {
    match pair.as_rule() {
        Rule::output => {
            let expr = pair.into_inner().next().unwrap().as_str().to_string();
            Ok(AstNode::Output(expr))
        }
        _ => Err(format!("Unsupported command: {:?}", pair)),
    }
}
