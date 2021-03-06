use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR").unwrap();

    tonic_build::configure()
        .build_client(false)
        .file_descriptor_set_path(format!("{}/lightspeed.bin", out_dir))
        .compile(
            &[
                "src/lightspeed/protocol/server.proto",
                "src/lightspeed/protocol/devices/device.proto",
            ],
            &["src/lightspeed/"],
        )?;
    Ok(())
}
