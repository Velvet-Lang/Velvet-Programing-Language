use crate::parser::AstNode;

pub fn optimize_ast(ast: Vec<AstNode>) -> Vec<AstNode> {
    let mut optimized = Vec::new();
    for node in ast {
        match node {
            AstNode::Comment(_) => {} // Dead code elimination
            AstNode::Loop(count, cmd) => {
                if let Ok(n) = count.parse::<u32>() {
                    if n <= 3 {
                        // Unroll small loops
                        for i in 0..n {
                            let mut new_cmd = cmd.clone();
                            if let AstNode::Output(ref mut s) = new_cmd.as_mut() {
                                *s = s.replace("$INDEX", &i.to_string());
                            }
                            optimized.push(*new_cmd);
                        }
                        continue;
                    }
                }
                optimized.push(AstNode::Loop(count, cmd));
            }
            _ => optimized.push(node),
        }
    }
    optimized
}
