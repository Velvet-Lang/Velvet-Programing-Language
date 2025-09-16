use pest::Parser;
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "../../core/velvet.pest"]
struct VelvetParser;

fn format_code(input: &str) -> String {
    let pairs = VelvetParser::parse(Rule::program, input).unwrap();
    let mut formatted = String::new();
    for pair in pairs {
        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::dependency => formatted.push_str(&format!("{}\n", inner.as_str())),
                Rule::COMMENT => formatted.push_str(&format!("{}\n", inner.as_str())),
                Rule::embed => {
                    let mut inner = inner.into_inner();
                    let lang = inner.next().unwrap().as_str();
                    let code = inner.next().unwrap().as_str();
                    formatted.push_str(&format!("#{} {{\n  {}\n}}\n", lang, code.trim()));
                }
                Rule::output => formatted.push_str(&format!("{}\n", inner.as_str())),
                Rule::if_stmt => formatted.push_str(&format!("{}\n", inner.as_str())),
                Rule::loop_stmt => formatted.push_str(&format!("{}\n", inner.as_str())),
                Rule::let_stmt => formatted.push_str(&format!("{}\n", inner.as_str())),
                Rule::match_stmt => {
                    let mut inner = inner.into_inner();
                    let value = inner.next().unwrap().as_str();
                    formatted.push_str(&format!("match {} {{\n", value));
                    for arm in inner {
                        let arm_str = arm.as_str().replace("->", " -> ");
                        formatted.push_str(&format!("  {}\n", arm_str));
                    }
                    formatted.push_str("}\n");
                }
                Rule::spawn_stmt => formatted.push_str(&format!("{}\n", inner.as_str())),
                Rule::try_stmt => formatted.push_str(&format!("{}\n", inner.as_str())),
                Rule::type_def => formatted.push_str(&format!("{}\n", inner.as_str())),
                Rule::trait_def => {
                    let mut inner = inner.into_inner();
                    let name = inner.next().unwrap().as_str();
                    formatted.push_str(&format!("trait {} {{\n", name));
                    for method in inner {
                        formatted.push_str(&format!("  {};\n", method.as_str()));
                    }
                    formatted.push_str("}\n");
                }
                _ => {}
            }
        }
    }
    formatted
}

fn main() {
    for entry in fs::read_dir("src").unwrap() {
        let path = entry.unwrap().path();
        if path.extension().map_or(false, |ext| ext == "vel") {
            let input = fs::read_to_string(&path).unwrap();
            let formatted = format_code(&input);
            fs::write(&path, formatted).unwrap();
            println!("Formatted {}", path.display());
        }
    }
}
