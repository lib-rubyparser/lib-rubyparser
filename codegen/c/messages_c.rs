use lib_ruby_parser_nodes::template::*;

const TEMPLATE: &str = "// This file is autogenerated by <helper generated-by>
#include \"structs.h\"

// drop variant fns
<each-message><dnl>
void LIB_RUBY_PARSER_drop_message_<helper message-lower-name>(LIB_RUBY_PARSER_<helper message-camelcase-name>* variant)
{
<each-message-field><dnl>
    <helper message-field-drop-fn-name>(&(variant-><helper message-field-c-name>));
</each-message-field><dnl>
<if message-has-no-fields><dnl>
    (void)variant;
<else><dnl>
</if><dnl>
}
</each-message><dnl>

void LIB_RUBY_PARSER_drop_diagnostic_message(LIB_RUBY_PARSER_DiagnosticMessage *message)
{
    switch(message->tag)
    {
<each-message><dnl>
    case LIB_RUBY_PARSER_MESSAGE_<helper message-upper-name>:
        LIB_RUBY_PARSER_drop_message_<helper message-lower-name>(&(message->as.<helper message-lower-name>));
        break;
</each-message><dnl>
    }
}
";

pub(crate) fn codegen() {
    let template = TemplateRoot::new(TEMPLATE).unwrap();
    let fns = crate::codegen::fns::default_fns!();

    let contents = template.render(ALL_DATA, &fns);
    std::fs::write("external/c/messages.c", contents).unwrap();
}
