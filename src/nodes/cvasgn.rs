use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Cvasgn {
    pub name: String,
    pub value: Option<Box<Node>>,

    pub name_l: Range,
    pub operator_l: Option<Range>,
    pub expression_l: Range,
}

impl InnerNode for Cvasgn {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_str(&self.name);
        if let Some(value) = &self.value {
            result.push_node(value);
        }
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "cvasgn"
    }
}
