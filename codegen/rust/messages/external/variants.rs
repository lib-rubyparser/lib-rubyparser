use lib_ruby_parser_bindings::helpers::messages::{
    field_getter as bindings_field_getter, variant_getter as bindings_variant_getter,
};

fn contents() -> String {
    let messages = lib_ruby_parser_nodes::messages();

    format!(
        "// This file is auto-generated by {generator}

use crate::DiagnosticMessage;
use crate::error::message::DiagnosticMessageBlob;
use crate::containers::ExternalStringPtr as StringPtr;
use crate::containers::StringPtrBlob;

type Byte = u8;
type ByteBlob = u8;

{variants}
",
        generator = file!(),
        variants = messages.map(&variant).join("\n\n")
    )
}

pub(crate) fn codegen() {
    std::fs::write("src/error/message/external/variants.rs", contents()).unwrap();
}

fn variant(message: &lib_ruby_parser_nodes::Message) -> String {
    let size_var = format!("MESSAGE_{}_SIZE", message.upper_name());

    format!(
        "use crate::containers::size::{size_var};
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct {struct_name}Blob {{
    blob: [u8; {size_var}],
}}

/// External {struct_name}
#[repr(C)]
pub struct {struct_name} {{
    pub(crate) blob: {struct_name}Blob,
}}

extern \"C\" {{
    fn {extern_variant_getter_name}(blob: *const DiagnosticMessageBlob) -> *const {struct_name}Blob;
    {extern_getters}
}}

impl DiagnosticMessage {{
    /// Casts `self` to `Option<&{struct_name}>`, return `None` if variant doesn't match
    pub fn as_{lower}(&self) -> Option<&{struct_name}> {{
        unsafe {{
            ({extern_variant_getter_name}(&self.blob) as *const {struct_name}).as_ref()
        }}
    }}
}}

impl {struct_name} {{
    {getters}
}}

impl std::fmt::Debug for {struct_name} {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        f.debug_struct(\"{struct_name}\")
            {debug_fields}
            .finish()
    }}
}}

impl PartialEq for {struct_name} {{
    #[allow(unused_variables)]
    fn eq(&self, other: &Self) -> bool {{
        {compare_fields}
    }}
}}
",
        size_var = size_var,
        struct_name = message.camelcase_name(),
        extern_getters = extern_getters(message),
        extern_variant_getter_name = bindings_variant_getter::name(message),
        lower = message.lower_name(),
        getters = getters(message),
        debug_fields = debug_fields(message),
        compare_fields = compare_fields(message)
    )
}

fn extern_getters(message: &lib_ruby_parser_nodes::Message) -> String {
    message
        .fields
        .map(&|field| {
            format!(
                "fn {name}(blob: *const {struct_name}Blob) -> *const {return_type}Blob;",
                name = bindings_field_getter::name(message, field),
                struct_name = message.camelcase_name(),
                return_type = field_type(field)
            )
        })
        .join("\n    ")
}

fn getters(message: &lib_ruby_parser_nodes::Message) -> String {
    message
        .fields
        .map(&|field| {
            format!(
                "/// Return `{field_name}` field
    pub fn get_{field_name}(&self) -> &{return_type} {{
        unsafe {{
            #[allow(trivial_casts)]
            ({extern_getter}(&self.blob) as *const {return_type}).as_ref().unwrap()
        }}
    }}",
                field_name = field.name,
                return_type = field_type(field),
                extern_getter = bindings_field_getter::name(message, field),
            )
        })
        .join("\n    ")
}
fn debug_fields(message: &lib_ruby_parser_nodes::Message) -> String {
    message
        .fields
        .map(&|field| {
            format!(
                ".field(\"{field_name}\", self.get_{field_name}())",
                field_name = field.name
            )
        })
        .join("\n            ")
}
fn compare_fields(message: &lib_ruby_parser_nodes::Message) -> String {
    let checks = message.fields.map(&|field| {
        format!(
            "self.get_{field_name}() == other.get_{field_name}()",
            field_name = field.name
        )
    });

    if checks.is_empty() {
        String::from("true")
    } else {
        checks.join(" && ")
    }
}

fn field_type(field: &lib_ruby_parser_nodes::MessageField) -> &str {
    match field.field_type {
        lib_ruby_parser_nodes::MessageFieldType::Str => "StringPtr",
        lib_ruby_parser_nodes::MessageFieldType::Byte => "Byte",
    }
}