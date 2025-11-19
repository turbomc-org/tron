fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::{env, path::PathBuf};

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let proto_dir = manifest_dir.join("proto");

    let proto_files: Vec<_> = std::fs::read_dir(&proto_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() && path.extension() == Some("proto".as_ref()) {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    if proto_files.is_empty() {
        eprintln!("No .proto files found in {}", proto_dir.display());
        std::process::exit(1);
    }

    println!(
        "\x1b[36m🔧 Compiling {} protobuf files\x1b[0m",
        proto_files.len()
    );

    tonic_prost_build::configure()
        .compile_protos(&proto_files, &[proto_dir.clone()])
        .map_err(|err| -> Box<dyn std::error::Error> {
            format!(
                "\x1b[31m❌ Protobuf compilation failed!\x1b[0m\n  \x1b[33mInclude:\x1b[0m {}\n  \x1b[33mError:\x1b[0m {}",
                proto_dir.display(),
                err
            )
            .into()
        })?;

    println!("\x1b[32m✅ Protobufs compiled successfully!\x1b[0m");

    Ok(())
}
