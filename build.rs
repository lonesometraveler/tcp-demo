use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["src/common/messages.proto"], &["src/"])?;
    Ok(())
}
