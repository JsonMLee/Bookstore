// for tonic-build
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/helloworld.proto")?;
    tonic_build::compile_protos("../proto/bookstore.proto")?;
    Ok(())
}