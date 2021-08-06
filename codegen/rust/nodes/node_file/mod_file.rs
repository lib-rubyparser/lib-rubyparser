fn contents(node: &lib_ruby_parser_nodes::Node) -> String {
    format!(
        "// This file is autogenerated by {generator}
#[cfg(feature = \"compile-with-external-structures\")]
mod external;
#[cfg(feature = \"compile-with-external-structures\")]
pub use external::{struct_name};

#[cfg(not(feature = \"compile-with-external-structures\"))]
mod native;
#[cfg(not(feature = \"compile-with-external-structures\"))]
pub use native::{struct_name};

mod internal;
pub(crate) use internal::Internal{struct_name};
",
        generator = file!(),
        struct_name = node.struct_name
    )
}

pub(crate) fn codegen(node: &lib_ruby_parser_nodes::Node) {
    let path = format!("src/nodes/types/{}/mod.rs", node.filename);
    std::fs::write(&path, contents(node)).unwrap();
}
