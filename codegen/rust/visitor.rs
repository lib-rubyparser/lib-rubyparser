use lib_ruby_parser_nodes::{template::*, NodeField};

const TEMPLATE: &str = "// This file is auto-generated by {{ helper generated-by }}

use crate::nodes::*;
use crate::traverse::visitor::{Item, Visit, Visitor};
use crate::Node;

/// Trait that must be implement to observe actions
/// that are performed by `Visitor` while it traverses given `Node`.
pub trait Observer {
{{ each node }}<dnl>
    /// Invoked by a `Visitor` on entering into `{{ helper node-camelcase-name }}` node.
    #[allow(unused_variables)]
    fn on_{{ helper node-lower-name }}(&mut self, node: &{{ helper node-camelcase-name }}) {}
{{ end }}

    /// Caled when entering any `Node`
    #[allow(unused_variables)]
    fn on_node(&mut self, node: &Node) {}

    /// Called when exiting any `Node`
    #[allow(unused_variables)]
    fn on_node_moving_up(&mut self, node: &Node) {}

    /// Called when entering any optional `Node`
    #[allow(unused_variables)]
    fn on_option_node(&mut self, node: &Option<Box<Node>>) {}

    /// Called when entering any `Vec<Node>`
    #[allow(unused_variables)]
    fn on_node_list(&mut self, nodes: &[Node]) {}

    /// Called when entering any AST node,
    /// `subitem` is different for different `Node` fields,
    /// check documentation of `traverse::visitor::Item`
    #[allow(unused_variables)]
    fn on_subitem(&mut self, subitem: Item) {}

    /// Called when exiting any AST node,
    /// `subitem` is different for different `Node` fields,
    /// check documentation of `traverse::visitor::Item`
    #[allow(unused_variables)]
    fn on_subitem_moving_up(&mut self, subitem: Item) {}
}

impl<TObserver: Observer> Visit<&Node> for Visitor<TObserver> {
    fn visit(&mut self, node: &Node, visit_as: Item) {
        self.observer.on_subitem(visit_as);
        self.observer.on_node(node);

        match node {
{{ each node }}<dnl>
            Node::{{ helper node-camelcase-name }}(inner) => {
                self.visit_{{ helper node-lower-name }}(inner)
            }
{{ end }}
        }

        self.observer.on_node_moving_up(node);
        self.observer.on_subitem_moving_up(visit_as);
    }
}

impl<T> Visitor<T>
where
    T: Observer,
{
{{ each node }}<dnl>
    fn visit_{{ helper node-lower-name }}(&mut self, node: &{{ helper node-camelcase-name }}) {
        self.observer.on_{{ helper node-lower-name }}(node);

{{ each node-field }}<dnl>
        {{ helper visit-children }}
{{ end }}<dnl>
    }
{{ end }}
}
";

pub(crate) fn codegen() {
    let template = TemplateRoot::new(TEMPLATE).unwrap();
    let mut fns = crate::codegen::fns::default_fns!();

    fns.register::<NodeField, F::Helper>("visit-children", local_helpers::visit_children);

    let contents = template.render(ALL_DATA, &fns);
    std::fs::write("src/traverse/visitor/visit_gen.rs", contents).unwrap();
}

mod local_helpers {
    use lib_ruby_parser_nodes::NodeField;

    pub(crate) fn visit_children(node_field: &NodeField) -> String {
        let node = &node_field.node;
        let field_name = crate::codegen::fns::rust::node_fields::rust_field_name(node_field);

        use lib_ruby_parser_nodes::NodeFieldType::*;
        match node_field.field_type {
            Node => {}
            Nodes => {}
            MaybeNode { .. } => {}

            Loc | MaybeLoc | Str { .. } | MaybeStr { .. } | StringValue | U8 => {
                return format!("// skip {}", field_name)
            }
        }

        let variant = {
            fn capitalize_field_name(s: &str) -> String {
                s.split("_").map(|word| capitalize_word(word)).collect()
            }

            fn capitalize_word(s: &str) -> String {
                let mut c = s.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            }

            match (&node.wqp_name[..], &field_name[..]) {
                (_, "statements") => "Stmts".to_string(),
                (_, "call") => "MethodCall".to_string(),
                (_, "default") => "DefaultValue".to_string(),
                (_, "items") => "MlhsItems".to_string(),
                ("when", "patterns") => "Args".to_string(),
                ("undef", "names") => "Args".to_string(),
                ("args", "args") => "Arglist".to_string(),
                ("procarg0", "args") => "Arglist".to_string(),
                ("rescue", "else_") => "ElseBody".to_string(),
                _ => capitalize_field_name(&field_name),
            }
        };

        format!(
            "self.visit(&node.{field_name}, Item::{variant});",
            field_name = field_name,
            variant = variant
        )
    }
}
