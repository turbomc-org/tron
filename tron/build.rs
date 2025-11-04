fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::{env, path::PathBuf};

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let proto_dir = manifest_dir.join("../proto");
    let proto_file = proto_dir.join("bridge.proto");

    if !proto_file.exists() {
        eprintln!(
            "\x1b[31m❌ Missing proto file:\x1b[0m {}\n\x1b[33mHint:\x1b[0m Make sure '../proto/bridge.proto' exists relative to tron/",
            proto_file.display()
        );
        std::process::exit(1);
    }

    println!(
        "\x1b[36m🔧 Compiling protobuf:\x1b[0m {}\n",
        proto_file.display()
    );

    tonic_prost_build::configure()
        .compile_protos(&[proto_file], &[proto_dir.clone()])
        .map_err(|err| -> Box<dyn std::error::Error>  {
            format!(
                "\x1b[31m❌ Protobuf compilation failed!\x1b[0m\n  \x1b[33mPath:\x1b[0m {}\n  \x1b[33mInclude:\x1b[0m {}\n  \x1b[33mError:\x1b[0m {}",
                proto_dir.display(),
                manifest_dir.display(),
                err
            )
            .into()
        })?;

    println!("\x1b[32m✅ Protobuf compiled successfully!\x1b[0m");

    Ok(())
}
