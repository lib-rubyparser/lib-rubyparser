use crate::codegen::rust::nodes::helpers::filename;
use lib_ruby_parser_nodes::{template::*, Node, NodeField};

const TEMPLATE: &str = r#"// This file is auto-generated by {{ helper generated-by }}

{{ helper imports }}

{{ helper node-comment }}
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct {{ helper node-camelcase-name }} {
{{ each node-field }}<dnl>
{{ helper node-field-comment }}
    pub {{ helper node-field-rust-field-name }}: {{ helper node-field-native-type }},

{{ end }}<dnl>
}

impl InnerNode for {{ helper node-camelcase-name }} {
    fn expression(&self) -> &Loc {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
{{ each node-field }}<dnl>
        {{ helper inspect-field }}
{{ end }}<dnl>
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "{{ helper node-str-type }}"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
{{ each node-field }}<dnl>
        {{ helper print-field-with-loc }}
{{ end }}<dnl>
    }
}
"#;

pub(crate) fn codegen(node: &lib_ruby_parser_nodes::Node) {
    let template = NodeTemplateRoot::new(TEMPLATE).unwrap();
    let mut fns = crate::codegen::fns::default_fns!();

    fns.register::<Node, F::Helper>("imports", local_helpers::imports);
    fns.register::<NodeField, F::Helper>(
        "node-field-native-type",
        local_helpers::native_field_type,
    );
    fns.register::<NodeField, F::Helper>("inspect-field", local_helpers::inspect_field);
    fns.register::<NodeField, F::Helper>(
        "print-field-with-loc",
        local_helpers::print_field_with_loc,
    );

    let contents = template.render(node, &fns);

    let dir = filename(node);
    let path = format!("src/nodes/types/{}.rs", dir);
    std::fs::write(&path, contents).unwrap();
}

mod local_helpers {
    use lib_ruby_parser_nodes::{node_has_field, Node, NodeField};

    pub(crate) fn imports(node: &Node) -> String {
        use lib_ruby_parser_nodes::NodeFieldType::*;

        let mut imports = vec![];
        imports.push("use crate::nodes::InnerNode;");
        imports.push("use crate::nodes::InspectVec;");
        imports.push("use crate::Loc;");

        if node_has_field!(node, Node | Nodes | MaybeNode { .. }) {
            imports.push("use crate::Node;");
        }

        if node_has_field!(node, StringValue) {
            imports.push("use crate::Bytes;");
        }

        imports.join("\n")
    }

    pub(crate) fn native_field_type(node_field: &NodeField) -> String {
        use lib_ruby_parser_nodes::NodeFieldType;
        match node_field.field_type {
            NodeFieldType::Node => "Box<Node>",
            NodeFieldType::Nodes => "Vec<Node>",
            NodeFieldType::MaybeNode { .. } => "Option<Box<Node>>",
            NodeFieldType::Loc => "Loc",
            NodeFieldType::MaybeLoc => "Option<Loc>",
            NodeFieldType::Str { .. } => "String",
            NodeFieldType::MaybeStr { .. } => "Option<String>",
            NodeFieldType::StringValue => "Bytes",
            NodeFieldType::U8 => "u8",
        }
        .to_string()
    }

    pub(crate) fn inspect_field(node_field: &NodeField) -> String {
        use lib_ruby_parser_nodes::NodeFieldType::*;

        let method_name = match node_field.field_type {
            Node => "push_node",
            Nodes => "push_nodes",
            MaybeNode { regexp_options } => {
                if regexp_options {
                    "push_regex_options"
                } else if node_field.always_print {
                    "push_maybe_node_or_nil"
                } else {
                    "push_maybe_node"
                }
            }
            Loc => return format!(""),
            MaybeLoc => return format!(""),
            Str { raw } => {
                if raw {
                    "push_raw_str"
                } else {
                    "push_str"
                }
            }
            MaybeStr { chars } => {
                if chars {
                    "push_chars"
                } else if node_field.always_print {
                    "push_maybe_str_or_nil"
                } else {
                    "push_maybe_str"
                }
            }
            StringValue => "push_string_value",
            U8 => "push_u8",
        };

        let field_name = crate::codegen::fns::rust::node_fields::rust_field_name(node_field);

        format!("result.{}(&self.{});", method_name, field_name)
    }

    pub(crate) fn print_field_with_loc(node_field: &NodeField) -> String {
        use lib_ruby_parser_nodes::NodeFieldType::*;
        let field_name = crate::codegen::fns::rust::node_fields::rust_field_name(node_field);

        match node_field.field_type {
            Node => format!(
                "self.{field_name}.inner_ref().print_with_locs();",
                field_name = field_name
            ),
            Nodes =>
                format!(
                    "for node in self.{field_name}.iter() {{ node.inner_ref().print_with_locs(); }}",
                    field_name = field_name
                ),
            MaybeNode { .. } => format!(
                "if let Some(node) = self.{field_name}.as_ref() {{ node.inner_ref().print_with_locs() }}",
                field_name = field_name
            ),
            Loc => format!(
                r#"self.{field_name}.print("{printable_field_name}");"#,
                field_name = field_name,
                printable_field_name = node_field
                    .snakecase_name
                    .strip_suffix("_l")
                    .expect("expected loc field to end with _l")
            ),
            MaybeLoc => format!(
                r#"if let Some(loc) = self.{field_name}.as_ref() {{ loc.print("{printable_field_name}") }}"#,
                field_name = field_name,
                printable_field_name = node_field
                    .snakecase_name
                    .strip_suffix("_l")
                    .expect("expected loc field to end with _l"),
            ),
            Str { .. } => format!(""),
            MaybeStr { .. } => format!(""),
            StringValue => format!(""),
            U8 => format!(""),
        }
    }
}
