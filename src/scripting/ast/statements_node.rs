use crate::scripting::ast::node::ExpressionNode;

pub struct StatementsNode {
    nodes: Vec<Box<dyn ExpressionNode>>
}

impl StatementsNode {
    pub fn new () -> Self {
        StatementsNode {
            nodes: vec![]
        }
    }

    pub fn add_node (&mut self, node: Box<dyn ExpressionNode>) {
        self.nodes.push(node);
    }
}

impl ExpressionNode for StatementsNode {}
