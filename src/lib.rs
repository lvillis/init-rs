//! # init-rs
//!
//! Use the world's best Cargo to install software, bypassing GitHub.

use std::fs;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    /// Test the ability to copy binary files on a Linux system.
    #[test]
    fn test_copy_binary() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path();
        let target_temp_dir = TempDir::new().unwrap();
        let target_dir = target_temp_dir.path();

        let binary_name = "test_binary";
        let source_file_path = source_dir.join(binary_name);

        // Create a temporary binary file to simulate copying
        let mut file = File::create(&source_file_path).unwrap();
        writeln!(file, "binary content").unwrap();

        let copy_result = copy_binary(binary_name, source_dir, &target_dir);

        assert!(copy_result.is_ok());
        assert!(target_dir.join(binary_name).exists());
    }
}
