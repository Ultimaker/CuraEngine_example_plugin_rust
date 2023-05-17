use std::io::Result;
use std::process::Command;

fn main() -> Result<()> {
    // Download the grpc definitions
    Command::new("git")
        .args([
            "clone",
            "git@github.com:Ultimaker/curaengine_grpc_defintions.git",
        ])
        .output()?;

    // Compile the proto files
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(
            &["plugin.proto", "simplify.proto", "postprocess.proto"],
            &["./curaengine_grpc_defintions"],
        )?;

    Ok(())
}
