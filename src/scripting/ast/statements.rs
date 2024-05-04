use super::expression::ExpressionNode;

pub struct StatementsNode {
    pub nodes: Vec<Box<dyn ExpressionNode>>,
}

impl StatementsNode {
    pub fn new() -> Self {
        StatementsNode { nodes: vec![] }
    }

    pub fn add_node(&mut self, node: Box<dyn ExpressionNode>) {
        self.nodes.push(node);
    }
}

impl ExpressionNode for StatementsNode {}

impl Default for StatementsNode {
    fn default() -> Self {
        Self::new()
    }
}
