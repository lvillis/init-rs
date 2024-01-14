use std::fs;
use std::env;
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

/// Copy the binary file to the specified directory.
///
/// # Parameters
///
/// * `binary_name` - The name of the binary file to copy.
/// * `source_dir` - The path to the source file directory.
/// * `target_dir` - The path to the target directory.
///
/// # Return
///
/// If successful, return the target path `PathBuf`。
/// On failure, returns a string describing the error.
///
/// # Example
///
/// ```
/// # {
/// let source_dir = std::path::Path::new(".");
/// let target_dir = std::path::Path::new("/usr/local/bin");
/// let result = init_rs::copy_binary("example_binary", source_dir, target_dir);
/// assert!(result.is_ok());
/// # }
/// ```
///
pub fn copy_binary(
    binary_name: &str,
    source_dir: &Path,
    target_dir: &Path,
) -> Result<PathBuf, String> {
    let source_path = source_dir.join(binary_name);
    let target_path = target_dir.join(binary_name);

    fs::copy(&source_path, &target_path)
        .map_err(|e| format!("Failed to copy binary file '{}': {}", binary_name, e))?;

    // chmod +x
    #[cfg(target_family = "unix")]
    {
        let permissions = fs::Permissions::from_mode(0o755);
        fs::set_permissions(&target_path, permissions)
            .map_err(|e| format!("Failed to set permissions for '{}': {}", binary_name, e))?;
    }

    Ok(target_path)
}

#[cfg(target_os = "linux")]
fn main() {
    // Skip binary installation when building documentation on docs.rs
    if env::var("DOCS_RS").is_ok() {
        println!("Skipping binary installation on docs.rs");
        return;
    }

    let source_dir = std::path::Path::new("bin");
    let target_dir = std::path::Path::new("/usr/local/bin");

    let features_and_binaries = vec![
        #[cfg(feature = "just")]
        ("just", "just"),
        #[cfg(feature = "fd")]
        ("fd", "fd"),
        #[cfg(feature = "lll")]
        ("lll", "lll"),
    ];

    let mut installed_any = false;

    for &(feature, binary_name) in &features_and_binaries {
        match copy_binary(binary_name, source_dir, target_dir) {
            Ok(path) => {
                println!("Binary file '{}' copied to: {:?}", binary_name, path);
                installed_any = true;
            }
            Err(e) => eprintln!("Failed to install '{}': {}", binary_name, e),
        }
    }

    if !installed_any {
        panic!("No feature selected or installation failed. Please ensure features are enabled and valid.");
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    if cfg!(debug_assertions) {
        println!("Running in local development (debug) mode on non-Linux OS...");
    } else {
        panic!("Non-Linux OS detected: {}", env::consts::OS);
    }
}
