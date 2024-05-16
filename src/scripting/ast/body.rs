use super::expression::ExpressionNode;

pub struct BodyNode {
    pub nodes: Vec<Box<dyn ExpressionNode>>,
}

impl BodyNode {
    pub fn new() -> Self {
        BodyNode { nodes: vec![] }
    }

    pub fn add_node(&mut self, node: Box<dyn ExpressionNode>) {
        self.nodes.push(node);
    }
}

impl ExpressionNode for BodyNode {
    fn get_type(&self) -> &'static str {
        stringify!(ExpressionNode)
    }
}

impl Default for BodyNode {
    fn default() -> Self {
        Self::new()
    }
}
