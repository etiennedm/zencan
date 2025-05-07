
fn main() {
    if let Err(e) = zencan_build::build_node_from_device_config("EXAMPLE1", "device_configs/example1.toml") {
        eprintln!("Error building node from example1.toml: {}", e);
        std::process::exit(1);
    }
    if let Err(e) = zencan_build::build_node_from_device_config("EXAMPLE2", "device_configs/example2.toml") {
        eprintln!("Error building node from example2.toml: {}", e);
        std::process::exit(1);
    }
}