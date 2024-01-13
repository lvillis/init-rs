//! # init-rs
//!
//! Use the world's best Cargo to install software, bypassing GitHub.

use std::path::{Path, PathBuf};
use std::fs;

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
/// # #[cfg(target_os = "linux")]
/// # {
/// let source_dir = std::path::Path::new(".");
/// let target_dir = std::path::Path::new("/usr/local/bin");
/// let result = init_rs::copy_binary("example_binary", source_dir, target_dir);
/// assert!(result.is_ok());
/// # }
/// ```
///
pub fn copy_binary(binary_name: &str, source_dir: &Path, target_dir: &Path) -> Result<PathBuf, String> {
    let source_path = source_dir.join(binary_name);
    let target_path = target_dir.join(binary_name);

    fs::copy(&source_path, &target_path)
        .map_err(|e| format!("Failed to copy binary file '{}': {}", binary_name, e))?;

    Ok(target_path)
}

#[cfg(test)]
mod tests {
    #[cfg(target_os = "linux")]
    use super::*;
    #[cfg(target_os = "linux")]
    use std::fs;
    #[cfg(target_os = "linux")]
    use std::fs::{self, File};
    #[cfg(target_os = "linux")]
    use std::io::Write;
    #[cfg(target_os = "linux")]
    use tempfile::tempdir;

    /// Test the ability to copy binary files on a Linux system.
    #[cfg(target_os = "linux")]
    #[test]
    fn test_copy_binary() {
        let temp_dir = tempdir().unwrap();
        let source_dir = temp_dir.path();
        let target_dir = tempdir().unwrap().path();

        let binary_name = "test_binary";
        let source_file_path = source_dir.join(binary_name);

        // Create a temporary binary file to simulate copying
        let mut file = File::create(&source_file_path).unwrap();
        writeln!(file, "binary content").unwrap();

        let copy_result = copy_binary(binary_name, source_dir, &target_dir);

        assert!(copy_result.is_ok());
        assert!(target_dir.join(binary_name).exists());

        // Cleanup: Delete temporary files created
        fs::remove_file(source_file_path).unwrap();
    }
}
