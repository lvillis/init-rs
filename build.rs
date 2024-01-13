#[cfg(target_os = "linux")]
fn main() {
    let binary_name = if cfg!(feature = "just") {
        "just"
    } else if cfg!(feature = "fd") {
        "fd"    
    } else {
        panic!("No feature selected. Please build with --features just or --features fd.");
    };

    let source_dir = std::path::Path::new("bin");
    let target_dir = std::path::Path::new("/usr/local/bin");

    match init_rs::copy_binary(binary_name, source_dir, target_dir) {
        Ok(path) => println!("Binary file '{}' copied to: {:?}", binary_name, path),
        Err(e) => panic!("{}", e),
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    if cfg!(debug_assertions) {
        println!("Running in local development (debug) mode on non-Linux OS...");
    } else {
        panic!("Non-Linux OS detected: {}", std::env::consts::OS);
    }
}
