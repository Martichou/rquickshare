extern crate prost_build;

use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Write;

fn main() {
    prost_build::compile_protos(
        &[
            "src/proto_src/device_to_device_messages.proto",
            "src/proto_src/offline_wire_formats.proto",
            "src/proto_src/securegcm.proto",
            "src/proto_src/securemessage.proto",
            "src/proto_src/ukey.proto",
            "src/proto_src/wire_format.proto",
        ],
        &["src/proto_src"],
    )
    .unwrap();

    let mut exports: Vec<_> = fs::read_dir("./bindings")
        .unwrap()
        .filter_map(Result::ok)
        .filter_map(|p| {
            p.path()
                .file_stem()
                .and_then(OsStr::to_str)
                .map(str::to_owned)
        })
        .filter(|f| f != "index")
        .map(|f| format!("export * from \"./{}\"", f))
        .collect();
    // Sort it to avoid having the index.ts being different for no reason
    exports.sort();

    let mut file = File::create("./bindings/index.ts").unwrap();
    file.write_all(exports.join("\n").as_bytes()).unwrap();
}
