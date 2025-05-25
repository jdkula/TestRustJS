use glob::glob;
use std::{
    io::Result,
    path::PathBuf,
};

fn main() -> Result<()> {
    let glob_result = glob("../proto/**/*.proto");

    let mut result: Vec<PathBuf> = Vec::new();

    for opt in glob_result.map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to glob proto files: {}", e),
        )
    })? {
        let path = opt.map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to read proto: {}", e),
            )
        })?;
        println!("cargo::rerun-if-changed={}", path.display());
        result.push(path);
    }

    println!("cargo::rerun-if-changed=build.rs");
    let mut cfg = prost_build::Config::new();
    cfg.include_file("_proto.rs").compile_protos(&result, &["../proto"])?;

    Ok(())
}
