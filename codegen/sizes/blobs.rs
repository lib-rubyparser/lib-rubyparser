use super::sizes;

fn contents() -> String {
    format!(
        "// This file is autogenerated by {generator}

use crate::containers::size;

{blobs}
",
        generator = file!(),
        blobs = blobs()
    )
}

pub(crate) fn codegen() {
    std::fs::write("src/blobs/gen.rs", contents()).unwrap();
}

fn blobs() -> String {
    let mut messages = vec![];
    let mut nodes = vec![];

    for size in sizes() {
        let size_name = size.name.clone();

        let struct_name = camelcase(size_name.strip_suffix("_SIZE").unwrap());

        if struct_name.starts_with("Node") && struct_name != "Node" {
            let struct_name = struct_name.strip_prefix("Node").unwrap().to_owned();
            nodes.push((struct_name, size_name));
        } else if struct_name.starts_with("Message") {
            let struct_name = struct_name.strip_prefix("Message").unwrap().to_owned();
            messages.push((struct_name, size_name));
        } else {
            // ignore
        }
    }

    format!(
        "/// Mod with node blobs
pub mod nodes {{
    use super::size;
    use crate::nodes::*;

    {node_blobs}
}}

/// Mod with message blobs
pub mod messages {{
    use super::size;
    use crate::error::message::variants::*;

    {message_blobs}
}}
",
        node_blobs = map_blobs(nodes, &blob_code).join("\n    "),
        message_blobs = map_blobs(messages, &blob_code).join("\n    "),
    )
}

fn camelcase(capitalized_name: &str) -> String {
    capitalized_name
        .split('_')
        .map(|word| {
            if word.is_empty() {
                format!("_")
            } else {
                format!("{}{}", word[..1].to_uppercase(), word[1..].to_lowercase())
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

fn map_blobs(blobs: Vec<(String, String)>, f: &dyn Fn(&str, &str) -> String) -> Vec<String> {
    blobs
        .into_iter()
        .map(|(struct_name, size_name)| f(&struct_name, &size_name))
        .collect()
}

fn blob_code(struct_name: &str, size_name: &str) -> String {
    let blob_name = format!("{}Blob", struct_name);

    format!(
        "declare_blob!(
        size = size::{size_name},
        value = {struct_name},
        blob = {blob_name},
        doc = \"Blob of the `{struct_name}`\"
    );",
        size_name = size_name,
        struct_name = struct_name,
        blob_name = blob_name
    )
}
