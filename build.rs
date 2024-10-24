use chrono::Utc;
use std::process::Command;

fn main() {
    // 获取 Git commit hash
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .expect("Failed to execute git command");
    let git_hash = String::from_utf8(output.stdout).unwrap().trim().to_string();

    // 获取构建日期
    let build_date = Utc::now().format("%Y-%m-%d").to_string();

    // 将这些信息传递给 main.rs
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=BUILD_DATE={}", build_date);
}
