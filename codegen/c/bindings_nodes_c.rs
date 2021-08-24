use crate::codegen::c::helpers;
use lib_ruby_parser_bindings::{
    helpers::nodes::{
        constructor::sig as external_constructor_sig,
        drop_variant::sig as external_drop_variant_sig,
        field_getter::sig as external_field_getter_sig,
        field_setter::sig as external_field_setter_sig,
        into_internal::sig as external_into_internal_sig,
        into_variant::sig as external_into_variant_sig,
        variant_getter::sig as external_variant_getter_sig,
        variant_predicate::sig as external_variant_predicate_sig,
    },
    Options,
};

fn contents(options: &Options) -> String {
    let nodes = lib_ruby_parser_nodes::nodes();

    format!(
        "// This file is autogenerated by {generator}

#include \"bindings.h\"
#include <stdio.h>

// Node constructors
{constructors}

// Node variant predicates
{variant_predicates}

// Node variant getter
{variant_getters}

// Node field getters
{field_getters}

// Node field setters
{field_setters}

// into_variant fns
{into_variant_fns}

// into_internal fns
{into_internal_fns}

// variant drop fns
{variant_drop_fns}

void lib_ruby_parser__internal__containers__node__drop(Node_BLOB* blob)
{{
    Node *node = (Node *)blob;
    drop_node(node);
}}
",
        generator = file!(),
        constructors = nodes.map(&|node| constructor(node, options)).join("\n"),
        variant_predicates = nodes
            .map(&|node| variant_predicate(node, options))
            .join("\n"),
        variant_getters = nodes.map(&|node| variant_getter(node, options)).join("\n"),
        field_getters = nodes
            .flat_map(&|node| field_getters(node, options))
            .join("\n"),
        field_setters = nodes
            .flat_map(&|node| field_setters(node, options))
            .join("\n"),
        into_internal_fns = nodes
            .map(&|node| into_internal_fn(node, options))
            .join("\n"),
        into_variant_fns = nodes.map(&|node| into_variant_fn(node, options)).join("\n"),
        variant_drop_fns = nodes.map(&|node| variant_drop_fn(node, options)).join("\n"),
    )
}

pub(crate) fn codegen(options: &Options) {
    std::fs::write("external/c/bindings_nodes.c", contents(options)).unwrap();
}

fn constructor(node: &lib_ruby_parser_nodes::Node, options: &Options) -> String {
    let fields = node
        .fields
        .map(&|field| {
            format!(
                ".{field_name} = {unpack}({field_name})",
                field_name = helpers::nodes::fields::field_name(field),
                unpack = helpers::nodes::fields::unpack_field_fn(field)
            )
        })
        .join(", ");

    format!(
        "{sig}
{{
    Node node = {{ .tag = {tag_name}, .as = {{ .{union_member} = {{ {fields} }} }} }};
    return PACK_Node(node);
}}",
        sig = external_constructor_sig(node, options),
        tag_name = helpers::nodes::enum_variant_name(node),
        union_member = helpers::nodes::union_member_name(node),
        fields = fields
    )
}
fn variant_predicate(node: &lib_ruby_parser_nodes::Node, options: &Options) -> String {
    format!(
        "{sig}
{{
    Node *node = (Node *)blob;
    return node->tag == {tag_name};
}}",
        sig = external_variant_predicate_sig(node, options),
        tag_name = helpers::nodes::enum_variant_name(node),
    )
}
fn variant_getter(node: &lib_ruby_parser_nodes::Node, options: &Options) -> String {
    format!(
        "{sig}
{{
    Node *node = (Node *)blob;
    if (node->tag != {tag_name}) {{
        return NULL;
    }}
    return ({struct_name}_BLOB *)(&(node->as.{union_member}));
}}",
        sig = external_variant_getter_sig(node, options),
        tag_name = helpers::nodes::enum_variant_name(node),
        struct_name = node.camelcase_name,
        union_member = helpers::nodes::union_member_name(node)
    )
}
fn field_getters(node: &lib_ruby_parser_nodes::Node, options: &Options) -> Vec<String> {
    node.fields.map(&|field| {
        let field_type = helpers::nodes::fields::field_type(field);

        format!(
            "{sig}
{{
    {variant} *variant = ({variant} *)blob;
    {field_type}* field = &(variant->{field_name});
    return ({blob_type} *)field;
}}",
            sig = external_field_getter_sig(node, field, options),
            variant = node.camelcase_name,
            field_type = field_type,
            field_name = helpers::nodes::fields::field_name(field),
            blob_type = helpers::nodes::fields::blob_type(field)
        )
    })
}
fn field_setters(node: &lib_ruby_parser_nodes::Node, options: &Options) -> Vec<String> {
    node.fields.map(&|field| {
        let drop_old_value_fn = match field.field_type {
            lib_ruby_parser_nodes::NodeFieldType::Node => "drop_node_ptr",
            lib_ruby_parser_nodes::NodeFieldType::Nodes => "drop_node_list",
            lib_ruby_parser_nodes::NodeFieldType::MaybeNode { .. } => "drop_maybe_node_ptr",
            lib_ruby_parser_nodes::NodeFieldType::Loc => "drop_loc",
            lib_ruby_parser_nodes::NodeFieldType::MaybeLoc => "drop_maybe_loc",

            lib_ruby_parser_nodes::NodeFieldType::Str { .. } => "drop_string_ptr",

            lib_ruby_parser_nodes::NodeFieldType::MaybeStr { .. } => "drop_maybe_string_ptr",
            lib_ruby_parser_nodes::NodeFieldType::StringValue => "drop_bytes",
            lib_ruby_parser_nodes::NodeFieldType::U8 => "drop_byte",
        };

        format!(
            "{sig}
{{
    {struct_name}* variant = ({struct_name} *)blob;
    {drop_old_value_fn}(&(variant->{field_name}));
    variant->{field_name} = {unpack_fn}({field_name});
}}",
            sig = external_field_setter_sig(node, field, options),
            struct_name = node.camelcase_name,
            field_name = helpers::nodes::fields::field_name(field),
            unpack_fn = helpers::nodes::fields::unpack_field_fn(field),
            drop_old_value_fn = drop_old_value_fn
        )
    })
}
fn into_internal_fn(node: &lib_ruby_parser_nodes::Node, options: &Options) -> String {
    let fields = node
        .fields
        .map(&|field| {
            let field_name = helpers::nodes::fields::field_name(field);

            format!(
                ".{field_name} = {pack_fn}(variant.{field_name})",
                field_name = field_name,
                pack_fn = helpers::nodes::fields::pack_field_fn(field)
            )
        })
        .join(", ");

    format!(
        "{sig} {{
    {struct_name} variant = UNPACK_{struct_name}(blob);
    Internal{struct_name} internal = {{ {fields} }};
    return internal;
}}",
        sig = external_into_internal_sig(node, options),
        struct_name = node.camelcase_name,
        fields = fields
    )
}

fn into_variant_fn(node: &lib_ruby_parser_nodes::Node, options: &Options) -> String {
    format!(
        "{sig} {{
    Node node = UNPACK_Node(blob);
    {struct_name} variant = node.as.{union_member_name};
    return PACK_{struct_name}(variant);
}}",
        sig = external_into_variant_sig(node, options),
        struct_name = node.camelcase_name,
        union_member_name = helpers::nodes::union_member_name(node)
    )
}

fn variant_drop_fn(node: &lib_ruby_parser_nodes::Node, options: &Options) -> String {
    format!(
        "{sig} {{
    {struct_name} *variant = ({struct_name} *)blob;
    drop_node_{lower}(variant);
}}",
        sig = external_drop_variant_sig(node, options),
        struct_name = node.camelcase_name,
        lower = node.lower_name()
    )
}