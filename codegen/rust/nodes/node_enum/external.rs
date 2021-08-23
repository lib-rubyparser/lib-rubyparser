use crate::codegen::rust::nodes::helpers::{blob_type, field_type, node_field_name, struct_name};

use lib_ruby_parser_bindings::helpers::nodes::{
    constructor::name as extern_constructor_name, into_variant::name as extern_into_variant_name,
    variant_getter::name as extern_variant_getter_name,
    variant_predicate::name as extern_variant_predicate_name,
};

fn contents() -> String {
    let nodes = lib_ruby_parser_nodes::nodes();

    format!(
        "// This file is autogenerated by {generator}

use crate::nodes::InnerNode;
use crate::nodes::*;
use crate::containers::size::NODE_SIZE;

use crate::Loc;
use crate::Bytes;
use crate::containers::ExternalMaybePtr as MaybePtr;
use crate::containers::ExternalPtr as Ptr;
use crate::containers::ExternalList as List;
use crate::containers::ExternalMaybeLoc as MaybeLoc;
use crate::containers::ExternalStringPtr as StringPtr;
use crate::containers::ExternalMaybeStringPtr as MaybeStringPtr;

use crate::loc::LocBlob;
use crate::bytes::BytesBlob;
use crate::containers::MaybePtrBlob;
use crate::containers::PtrBlob;
use crate::containers::ListBlob;
use crate::containers::MaybeLocBlob;
use crate::containers::StringPtrBlob;
use crate::containers::MaybeStringPtrBlob;

use crate::containers::IntoBlob;

type Byte = u8;
type ByteBlob = u8;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct NodeBlob {{
    blob: [u8; NODE_SIZE],
}}

/// Generic combination of all known nodes.
#[repr(C)]
pub struct Node {{
    pub(crate) blob: NodeBlob,
}}

impl std::fmt::Debug for Node {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        {debug_impl}
    }}
}}

impl Clone for Node {{
    fn clone(&self) -> Self {{
        {clone_impl}
    }}
}}

impl PartialEq for Node {{
    fn eq(&self, other: &Self) -> bool {{
        {partial_eq_impl}
    }}
}}

impl IntoBlob for Node {{
    type Output = NodeBlob;

    fn into_blob(self) -> Self::Output {{
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }}
}}

impl Node {{
    pub(crate) fn inner_ref(&self) -> &dyn InnerNode {{
        {inner_ref}

        panic!(\"bug: unknown node type\")
    }}

    // make_<node> FNs
    {constructors}

    // is_<node> FNs

    {is_variant_fns}

    // as_<node> FNs
    {as_variant_fns}

    // as_<node>_mut FNs
    {as_variant_mut_fns}

    // into_<node> FNs
    {into_variant_fns}
}}

use crate::nodes::blobs::*;
extern \"C\"
{{
    {extern_fns}

    fn lib_ruby_parser__internal__containers__node__drop(blob: *mut NodeBlob);
}}

impl Drop for Node {{
    fn drop(&mut self) {{
        unsafe {{ lib_ruby_parser__internal__containers__node__drop(&mut self.blob) }}
    }}
}}
",
        generator = file!(),
        // trait impls
        debug_impl = debug_impl(&nodes),
        clone_impl = clone_impl(&nodes),
        partial_eq_impl = partial_eq_impl(&nodes),
        // fns
        inner_ref = nodes.map(&inner_ref).join("\n        "),
        constructors = nodes.map(&constructor).join("\n    "),
        is_variant_fns = nodes.map(&is_variant_fn).join("\n    "),
        as_variant_fns = nodes.map(&as_variant_fn).join("\n    "),
        as_variant_mut_fns = nodes.map(&as_variant_mut_fn).join("\n    "),
        into_variant_fns = nodes.map(&into_variant_fn).join("\n    "),
        // extern fns
        extern_fns = nodes.flat_map(&extern_fns).join("\n    ")
    )
}

pub(crate) fn codegen() {
    std::fs::write("src/nodes/node_enum/external.rs", contents()).unwrap();
}

fn debug_impl(nodes: &lib_ruby_parser_nodes::NodeList) -> String {
    let branches = nodes
        .map(&|node| {
            format!(
                "if let Some(inner) = self.as_{lower}() {{
            write!(f, \"{struct_name}({{:?}})\", inner)
        }}",
                lower = node.lower_name(),
                struct_name = struct_name(node)
            )
        })
        .join(" else ");

    format!(
        "{branches} else {{
            panic!(\"bug: unknown node type\")
        }}",
        branches = branches
    )
}
fn clone_impl(nodes: &lib_ruby_parser_nodes::NodeList) -> String {
    let branches = nodes
        .map(&|node| {
            let clone_fields = node
                .fields
                .map(&|field| format!("inner.get_{}().clone()", field.field_name))
                .join(", ");

            format!(
                "if let Some(inner) = self.as_{lower}() {{
            Self::make_{lower}({clone_fields})
        }}",
                lower = node.lower_name(),
                clone_fields = clone_fields
            )
        })
        .join(" else ");

    format!(
        "{branches} else {{
            panic!(\"bug: unknown node type\")
        }}",
        branches = branches
    )
}
fn partial_eq_impl(nodes: &lib_ruby_parser_nodes::NodeList) -> String {
    let branches = nodes
        .map(&|node| {
            format!(
                "if let Some(lhs) = self.as_{lower}() {{
            if let Some(rhs) = other.as_{lower}() {{
                lhs == rhs
            }} else {{
                false
            }}
        }}",
                lower = node.lower_name(),
            )
        })
        .join(" else ");

    format!(
        "{branches} else {{
            panic!(\"bug: unknown node type\")
        }}",
        branches = branches
    )
}

fn inner_ref(node: &lib_ruby_parser_nodes::Node) -> String {
    format!(
        "if let Some(inner) = self.as_{lower}() {{
            return inner;
        }}",
        lower = node.lower_name()
    )
}
fn constructor(node: &lib_ruby_parser_nodes::Node) -> String {
    let arglist = node
        .fields
        .map(&|field| {
            format!(
                "{name}: {t}",
                name = node_field_name(field),
                t = field_type(field)
            )
        })
        .join(", ");

    let fields = node
        .fields
        .map(&|field| format!("{}.into_blob()", node_field_name(field)))
        .join(", ");

    format!(
        "/// Constructs `Node::{node_type}` variant
    pub(crate) fn make_{lower_node_type}({arglist}) -> Self {{
        let blob = unsafe {{ {extern_constructor}({fields}) }};
        Self {{ blob }}
    }}",
        node_type = struct_name(node),
        lower_node_type = node.lower_name(),
        extern_constructor = extern_constructor_name(node),
        arglist = arglist,
        fields = fields
    )
}
fn is_variant_fn(node: &lib_ruby_parser_nodes::Node) -> String {
    format!(
        "/// Returns true if `self` is `Node::{node_type}`
    pub fn is_{lower_node_type}(&self) -> bool {{
        unsafe {{ {extern_fn_name}(&self.blob) }}
    }}",
        node_type = struct_name(node),
        lower_node_type = node.lower_name(),
        extern_fn_name = extern_variant_predicate_name(node)
    )
}
fn as_variant_fn(node: &lib_ruby_parser_nodes::Node) -> String {
    format!(
        "/// Casts `&Node` to `Option<&nodes::{node_type}>`
    pub fn as_{lower}(&self) -> Option<&{node_type}> {{
        unsafe {{ ({extern_fn_name}(&self.blob) as *const {node_type}).as_ref() }}
    }}",
        node_type = struct_name(node),
        lower = node.lower_name(),
        extern_fn_name = extern_variant_getter_name(node)
    )
}
fn as_variant_mut_fn(node: &lib_ruby_parser_nodes::Node) -> String {
    format!(
        "/// Casts `&Node` to `Option<&mut nodes::{node_type}>`
    pub fn as_{lower}_mut(&mut self) -> Option<&mut {node_type}> {{
        unsafe {{ ({extern_fn_name}(&mut self.blob) as *mut {node_type}).as_mut() }}
    }}",
        node_type = struct_name(node),
        lower = node.lower_name(),
        extern_fn_name = extern_variant_getter_name(node)
    )
}
fn into_variant_fn(node: &lib_ruby_parser_nodes::Node) -> String {
    format!(
        "/// Casts `self` to nodes::{node_type}, panics if variant doesn't match
    pub fn into_{lower}(self) -> {node_type} {{
        let blob = unsafe {{ {into_variant_fn_name}(self.into_blob()) }};
        {node_type} {{ blob }}
    }}",
        node_type = struct_name(node),
        lower = node.lower_name(),
        into_variant_fn_name = extern_into_variant_name(node),
    )
}

fn extern_fns(node: &lib_ruby_parser_nodes::Node) -> Vec<String> {
    let mut result = vec![];

    // constructor
    {
        let ctor_args = node
            .fields
            .map(&|field| format!("{}: {}", node_field_name(field), blob_type(field)))
            .join(", ");
        result.push(format!(
            "fn {name}({ctor_args}) -> NodeBlob;",
            name = extern_constructor_name(node),
            ctor_args = ctor_args,
        ));
    }

    // variant predicates
    {
        result.push(format!(
            "fn {name}(blob_ptr: *const NodeBlob) -> bool;",
            name = extern_variant_predicate_name(node)
        ))
    }

    // variant getters
    {
        result.push(format!(
            "fn {name}(blob_ptr: *const NodeBlob) -> *mut {node_type}Blob;",
            name = extern_variant_getter_name(node),
            node_type = struct_name(node)
        ))
    }

    // into_internal fn
    {
        let line = format!(
            "fn {fn_name}(blob: NodeBlob) -> {struct_name}Blob;",
            fn_name = extern_into_variant_name(node),
            struct_name = struct_name(node)
        );
        result.push(line);
    }

    result
}
