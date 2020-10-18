use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Pair {
    pub key: Box<Node>,
    pub value: Box<Node>,

    pub operator_l: Range,
    pub expression_l: Range,
}

impl InnerNode for Pair {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.key);
        result.push_node(&self.value);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "pair"
    }
}