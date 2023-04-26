// This is the build script for Cargo and it will configure tonic build

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Here we are telling tonic-build to compile our protocol buffers
    // which are located in the proto directory.
    tonic_build::compile_protos("proto/payments.proto")?;
    Ok(())
}