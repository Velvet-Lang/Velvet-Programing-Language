use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, multispace0},
    combinator::{map, opt},
    multi::{many0, separated_list0},
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, Clone)]
pub enum AstNode {
    Dependency(String),
    Comment(String),
    Embed(String, String),  // Language, Code
    Command(String),
    Output(String),
}

pub fn parse_velvet(input: &str) -> IResult<&str, Vec<AstNode>> {
    many0(alt((
        parse_dependency,
        parse_comment,
        parse_embed,
        parse_command,
        parse_output,
    )))(input)
}

fn parse_dependency(input: &str) -> IResult<&str, AstNode> {
    delimited(tag("["), map(alpha1, |s: &str| AstNode::Dependency(s.to_string())), tag("]"))(input)
}

fn parse_comment(input: &str) -> IResult<&str, AstNode> {
    preceded(tag("@"), map(take_while(|c| c != '\n'), |s: &str| AstNode::Comment(s.to_string())))(input)
}

fn parse_embed(input: &str) -> IResult<&str, AstNode> {
    let (input, lang) = preceded(tag("#"), alpha1)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, code) = delimited(tag("{"), take_while(|c| c != '}'), tag("}"))(input)?;
    Ok((input, AstNode::Embed(lang.to_string(), code.to_string())))
}

fn parse_command(input: &str) -> IResult<&str, AstNode> {
    map(take_while(|c| c != '\n'), |s: &str| AstNode::Command(s.to_string()))(input)
}

fn parse_output(input: &str) -> IResult<&str, AstNode> {
    preceded(tag("[Output] >"), map(take_while(|c| c != '|'), |s: &str| AstNode::Output(s.trim().to_string())))(input)
}
