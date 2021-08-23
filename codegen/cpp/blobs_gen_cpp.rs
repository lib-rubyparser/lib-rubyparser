fn contents() -> String {
    format!(
        "// This file is autogenerated by {generator}

#include \"blobs_gen.hpp\"
#include \"impl_blob.hpp\"

{node_blobs}

{message_blobs}
",
        generator = file!(),
        // blobs
        node_blobs = lib_ruby_parser_nodes::nodes().map(&node_blob).join("\n"),
        message_blobs = lib_ruby_parser_nodes::messages()
            .map(&message_blob)
            .join("\n"),
    )
}

pub(crate) fn codegen() {
    std::fs::write("external/cpp/blobs_gen.cpp", contents()).unwrap();
}

fn node_blob(node: &lib_ruby_parser_nodes::Node) -> String {
    format!("IMPL_BLOB({});", node.camelcase_name)
}
fn message_blob(message: &lib_ruby_parser_nodes::Message) -> String {
    format!("IMPL_BLOB({});", message.camelcase_name())
}
