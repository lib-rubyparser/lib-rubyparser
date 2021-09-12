use crate::codegen::rust::nodes::helpers::{node_field_name, struct_name};

fn contents(node: &lib_ruby_parser_nodes::Node) -> String {
    format!(
        "// This file is autogenerated by {generator}

{imports}

{comment}
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct {struct_name} {{
{fields_declaration}
}}

impl {struct_name} {{
    {getters}

    {setters}

    #[allow(dead_code)]
    pub(crate) fn into_internal(self) -> super::Internal{struct_name} {{
        let Self {{ {field_names} }} = self;
        super::Internal{struct_name} {{ {field_names} }}
    }}
}}

impl InnerNode for {struct_name} {{
    fn expression(&self) -> &Loc {{
        &self.expression_l
    }}

    fn inspected_children(&self, indent: usize) -> Vec<String> {{
        let mut result = InspectVec::new(indent);
        {inspected_children}
        result.strings()
    }}

    fn str_type(&self) -> &'static str {{
        \"{str_type}\"
    }}

    fn print_with_locs(&self) {{
        println!(\"{{}}\", self.inspect(0));
        {print_with_locs}
    }}
}}
",
        generator = file!(),
        imports = imports(&node).join("\n"),
        comment = node.render_comment("///", 0),
        struct_name = struct_name(node),
        fields_declaration = node.fields.map(field_declaration).join("\n\n"),
        inspected_children = node.fields.map(inspect_field).join("\n        "),
        str_type = node.wqp_name,
        print_with_locs = node.fields.flat_map(print_with_locs).join("\n        "),
        getters = node.fields.map(getter).join("\n\n    "),
        setters = node.fields.map(setter).join("\n\n    "),
        field_names = node.fields.map(|field| node_field_name(field)).join(", ")
    )
}

pub(crate) fn codegen(node: &lib_ruby_parser_nodes::Node) {
    let dir = super::filename(node);
    let path = format!("src/nodes/types/{}/native.rs", dir);
    std::fs::write(&path, contents(node)).unwrap();
}

fn imports(node: &lib_ruby_parser_nodes::Node) -> Vec<&str> {
    let mut imports = vec![];
    imports.push("use crate::nodes::InnerNode;");
    imports.push("use crate::nodes::InspectVec;");
    imports.push("use crate::Loc;");

    let has_field = |field_type: lib_ruby_parser_nodes::NodeFieldType| {
        node.fields.any_field_has_type(field_type)
    };

    if has_field(lib_ruby_parser_nodes::NodeFieldType::Node)
        || has_field(lib_ruby_parser_nodes::NodeFieldType::Nodes)
        || has_field(lib_ruby_parser_nodes::NodeFieldType::MaybeNode {
            regexp_options: true,
        })
        || has_field(lib_ruby_parser_nodes::NodeFieldType::MaybeNode {
            regexp_options: false,
        })
    {
        imports.push("use crate::Node;");
    }

    if has_field(lib_ruby_parser_nodes::NodeFieldType::StringValue) {
        imports.push("use crate::Bytes;");
    }

    imports
}

fn field_type(field: &lib_ruby_parser_nodes::NodeField) -> &str {
    use lib_ruby_parser_nodes::NodeFieldType;
    match field.field_type {
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
}

fn field_declaration(field: &lib_ruby_parser_nodes::NodeField) -> String {
    format!(
        "{comment}
    pub {field_name}: {field_type},",
        comment = field.render_comment("///", 4),
        field_name = node_field_name(field),
        field_type = field_type(field)
    )
}

fn inspect_field(field: &lib_ruby_parser_nodes::NodeField) -> String {
    use lib_ruby_parser_nodes::NodeFieldType;

    let method_name = match field.field_type {
        NodeFieldType::Node => "push_node",
        NodeFieldType::Nodes => "push_nodes",
        NodeFieldType::MaybeNode { regexp_options } => {
            if regexp_options {
                "push_regex_options"
            } else if field.always_print {
                "push_maybe_node_or_nil"
            } else {
                "push_maybe_node"
            }
        }
        NodeFieldType::Loc => return format!(""),
        NodeFieldType::MaybeLoc => return format!(""),
        NodeFieldType::Str { raw } => {
            if raw {
                "push_raw_str"
            } else {
                "push_str"
            }
        }
        NodeFieldType::MaybeStr { chars } => {
            if chars {
                "push_chars"
            } else {
                "push_maybe_str"
            }
        }
        NodeFieldType::StringValue => "push_string_value",
        NodeFieldType::U8 => "push_u8",
    };

    format!("result.{}(&self.{});", method_name, node_field_name(field))
}
fn print_with_locs(field: &lib_ruby_parser_nodes::NodeField) -> Vec<String> {
    use lib_ruby_parser_nodes::NodeFieldType;

    match field.field_type {
        NodeFieldType::Node => vec![format!(
            "self.{field_name}.inner_ref().print_with_locs();",
            field_name = node_field_name(field)
        )],
        NodeFieldType::Nodes => vec![
            format!(
                "for node in self.{field_name}.iter() {{",
                field_name = node_field_name(field)
            ),
            "  node.inner_ref().print_with_locs();".to_string(),
            "}".to_string(),
        ],
        NodeFieldType::MaybeNode { .. } => vec![format!(
            "self.{field_name}.as_ref().map(|node| node.inner_ref().print_with_locs());",
            field_name = node_field_name(field)
        )],
        NodeFieldType::Loc => vec![format!(
            "self.{field_name}.print(\"{printable_field_name}\");",
            field_name = node_field_name(field),
            printable_field_name = node_field_name(field)
                .strip_suffix("_l")
                .expect("expected loc field to end with _l")
        )],
        NodeFieldType::MaybeLoc => vec![format!(
            "self.{field_name}.as_ref().map(|loc| loc.print(\"{printable_field_name}\"));",
            field_name = node_field_name(field),
            printable_field_name = node_field_name(field)
                .strip_suffix("_l")
                .expect("expected loc field to end with _l"),
        )],
        NodeFieldType::Str { .. } => vec![],
        NodeFieldType::MaybeStr { .. } => vec![],
        NodeFieldType::StringValue => vec![],
        NodeFieldType::U8 => vec![],
    }
}

fn getter(field: &lib_ruby_parser_nodes::NodeField) -> String {
    format!(
        "#[doc(hidden)]
    pub fn get_{method_name}(&self) -> &{return_type} {{
        &self.{field_name}
    }}

    #[doc(hidden)]
    pub fn get_{method_name}_mut(&mut self) -> &mut {return_type} {{
        &mut self.{field_name}
    }}",
        method_name = field.field_name,
        field_name = node_field_name(field),
        return_type = field_type(field),
    )
}

fn setter(field: &lib_ruby_parser_nodes::NodeField) -> String {
    format!(
        "#[doc(hidden)]
    pub fn set_{method_name}(&mut self, value: {field_type}) {{
        self.{field_name} = value;
    }}",
        field_name = node_field_name(field),
        field_type = field_type(field),
        method_name = field.field_name,
    )
}
