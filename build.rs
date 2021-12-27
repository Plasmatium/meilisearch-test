use std::io::Result;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/searcher.proto");
    prost_build::compile_protos(&["src/searcher.proto"], &["src/"])?;
    Ok(())
}