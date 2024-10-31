use chrono::Local;
use std::process::Command;

fn main() {
    // 获取 git 信息
    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let git_branch = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let git_tag = Command::new("git")
        .args(["describe", "--tags", "--abbrev=0"])
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    // 获取 Rust 和 Cargo 版本
    let rust_version = Command::new("rustc")
        .arg("--version")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let cargo_version = Command::new("cargo")
        .arg("--version")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    // 获取构建时间
    let build_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // 输出所有变量供编译时使用
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=GIT_BRANCH={}", git_branch);
    println!("cargo:rustc-env=GIT_TAG={}", git_tag);
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);
    println!("cargo:rustc-env=RUST_VERSION={}", rust_version);
    println!("cargo:rustc-env=CARGO_VERSION={}", cargo_version);
}
