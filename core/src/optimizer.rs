use crate::parser::AstNode;

pub fn optimize_ast(ast: Vec<AstNode>) -> Vec<AstNode> {
    // Dead code elim: Remove comments
    ast.into_iter().filter(|node| !matches!(node, AstNode::Comment(_))).collect()
    // Inlining: Stub for now
}
