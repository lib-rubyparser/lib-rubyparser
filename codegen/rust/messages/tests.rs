use lib_ruby_parser_nodes::template::*;

const TEMPLATE: &str = "// This file is auto-generated by <helper generated-by>

use super::DiagnosticMessage;
crate::use_native_or_external!(StringPtr);

<each-message><dnl>
#[test]
fn test_<helper message-lower-name>() {
    let message = DiagnosticMessage::new_<helper message-lower-name>(
<each-message-field><dnl>
        <helper new-field-fn>(),
</each-message-field><dnl>
    );
    let variant = message.as_<helper message-lower-name>().unwrap();
<each-message-field><dnl>
    assert_eq!(variant.get_<helper message-field-name>(), &<helper new-field-fn>());
</each-message-field><dnl>
    drop(variant);
    drop(message);
}
</each-message><dnl>

fn new_str() -> StringPtr {
    StringPtr::from(String::from(\"foo\"))
}
fn new_byte() -> u8 {
    42
}
";

pub(crate) fn codegen() {
    let template = TemplateRoot::new(TEMPLATE).unwrap();
    let mut fns = crate::codegen::fns::default_fns!();

    fns.register_helper("new-field-fn", local_helpers::new_field_fn);

    let contents = template.render(ALL_DATA, &fns);
    std::fs::write("src/error/message/tests.rs", contents).unwrap();
}

mod local_helpers {
    pub(crate) fn new_field_fn(
        message_with_field: &lib_ruby_parser_nodes::MessageWithField,
    ) -> String {
        match message_with_field.field.field_type {
            lib_ruby_parser_nodes::MessageFieldType::Str => "new_str",
            lib_ruby_parser_nodes::MessageFieldType::Byte => "new_byte",
        }
        .to_string()
    }
}
