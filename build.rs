fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/")
        .build_client(false)
        .file_descriptor_set_path("src/lightspeed.bin")
        .compile(
            &[
                "src/lightspeed/protocol/server.proto",
                "src/lightspeed/protocol/devices/device.proto",
            ],
            &["src/lightspeed/"],
        )?;
    Ok(())
}
