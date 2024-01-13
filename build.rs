use std::fs;
use std::path::Path;

fn main() {
    let (source_binary_path, target_path) = if cfg!(target_os = "windows") {
        (Path::new("bin/just.exe"), Path::new("C:/Users/root/Documents/just.exe"))
    } else if cfg!(target_os = "linux") {
        (Path::new("bin/just"), Path::new("/root/just"))
    } else {
        // 如果操作系统既不是 Windows 也不是 Linux，退出构建脚本
        panic!("Unsupported operating system");
    };

    // 复制二进制文件
    if let Err(e) = fs::copy(source_binary_path, &target_path) {
        panic!("Failed to copy binary file: {}", e);
    }

    println!("Binary file copied to: {:?}", target_path);
}
