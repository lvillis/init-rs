#[cfg(target_os = "linux")]
use std::path::Path;
#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "windows")]
use std::env;


fn main() {
    #[cfg(target_os = "linux")]
    {
        let binary_name = if cfg!(feature = "just") {
            "just"
        } else if cfg!(feature = "fd") {
            "fd"
        } else {
            panic!("No feature selected. Please build with --features just or --features fd.");
        };

        let source_binary_path = Path::new("bin").join(binary_name);
        let target_path = Path::new("/root").join(binary_name);

        if let Err(e) = fs::copy(&source_binary_path, &target_path) {
            panic!("Failed to copy binary file '{}': {}", binary_name, e);
        }

        println!("Binary file '{}' copied to: {:?}", binary_name, target_path);
    }

    #[cfg(not(target_os = "linux"))]
    {
        if cfg!(debug_assertions) {
            println!("Running in local development (debug) mode on non-Linux OS...");
        } else {
            //panic!("This package is specifically designed for Linux OS.");
            panic!("Non-Linux OS detected: {}", env::consts::OS);
        }
    }
}
