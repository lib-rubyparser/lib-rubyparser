use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Mlhs {
    pub items: Vec<Node>,

    pub begin_l: Option<Range>,
    pub end_l: Option<Range>,
    pub expression_l: Range,
}

impl InnerNode for Mlhs {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_nodes(&self.items);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "mlhs"
    }
}