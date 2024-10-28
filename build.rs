use chrono::Utc;
use std::process::Command;

fn main() {
    let git_hash = get_git_hash().unwrap_or_else(|_| String::from("unknown"));
    let build_date = Utc::now().format("%Y-%m-%d").to_string();

    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=BUILD_DATE={}", build_date);
}

fn get_git_hash() -> Result<String, Box<dyn std::error::Error>> {
    match Command::new("git").args(["rev-parse", "HEAD"]).output() {
        Ok(output) if output.status.success() => {
            let hash = String::from_utf8_lossy(&output.stdout);
            Ok(hash[..8].to_string())
        }
        _ => Ok("unknown".to_string()), // 如果获取git hash失败，返回 "unknown"
    }
}
