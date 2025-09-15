// velvet-core/src/parser.rs
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, alphanumeric1, char, multispace0, one_of},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};
use crate::VelvetAst;

fn parse_command(i: &str) -> IResult<&str, VelvetAst> {
    let (i, cmd) = alpha1(i)?;
    let (i, _) = multispace0(i)?;
    let (i, args) = separated_list1(multispace0, alphanumeric1)(i)?;
    map(tuple((tag(cmd), args)), |t| VelvetAst::Command(t.0.to_string(), t.1.iter().map(|s| s.to_string()).collect()))(i)
}

fn parse_dependency(i: &str) -> IResult<&str, VelvetAst> {
    map(delimited(tag("["), take_until("]"), tag("]")), |dep| VelvetAst::Dependency(dep.to_string()))(i)
}

fn parse_comment(i: &str) -> IResult<&str, VelvetAst> {
    map(preceded(tag("@"), take_until("\n")), |c| VelvetAst::Comment(c.to_string()))(i)
}

fn parse_redir(i: &str) -> IResult<&str, VelvetAst> {
    map(tuple((one_of("><"), multispace0, alphanumeric1)), |t| {
        VelvetAst::IoRedir(t.0.to_string(), t.2.to_string())
    })(i)
}

// Główny parser
pub fn velvet_parser(i: &str) -> IResult<&str, Vec<VelvetAst>> {
    separated_list1(multispace0, alt((parse_command, parse_dependency, parse_comment, parse_redir)))(i)
}
