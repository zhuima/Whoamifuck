use anyhow::Result;
use colored::Colorize;
use self_update::cargo_crate_version;
use semver::Version;

#[allow(clippy::format_in_format_args)]
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::unused_async)]
#[allow(clippy::redundant_else)]
// 检查是否有新版本可用，但不自动更新
pub async fn check_version() -> Result<bool> {
    print!("{}", "Checking for updates... ".bright_blue().bold());

    let updater = self_update::backends::github::Update::configure()
        .repo_owner("zhuima")
        .repo_name("Whoamifuck")
        .bin_name("whoamifuck")
        .current_version(cargo_crate_version!())
        .build()?;

    match updater.get_latest_release() {
        Ok(latest) => {
            let current_version = Version::parse(cargo_crate_version!())?;
            let latest_version = Version::parse(&latest.version)?;

            if latest_version > current_version {
                println!("{}", "✨ New version found! ✨".bright_green().bold());
                println!(
                    "{}",
                    "┌─────────────────────────────────────────┐"
                        .bright_blue()
                        .bold()
                );
                println!(
                    "{} {}",
                    "│ Current version:".bright_blue().bold(),
                    current_version.to_string().bright_yellow().bold()
                );
                println!(
                    "{} {}",
                    "│ Latest version:".bright_blue().bold(),
                    latest_version.to_string().bright_green().bold()
                );
                println!(
                    "{} {}",
                    "│ Update command:".bright_blue().bold(),
                    "whoamifuck update".bright_cyan().bold()
                );
                println!(
                    "{}",
                    "└─────────────────────────────────────────┘"
                        .bright_blue()
                        .bold()
                );
                Ok(latest_version > current_version)
            } else {
                println!(
                    "{}",
                    "✓ You are using the latest version!".bright_green().bold()
                );
                Ok(false)
            }
        }
        Err(e) => {
            println!("{}", "✗ Failed to check for updates!".bright_red().bold());
            eprintln!("Error details: {e}");
            // 如果是网络超时或 GitHub API 限制，给出更友好的提示
            if e.to_string().contains("timeout") {
                eprintln!("Network timeout. Please check your internet connection.");
            } else if e.to_string().contains("rate limit") {
                eprintln!("GitHub API rate limit exceeded. Please try again later.");
            }
            Ok(false)
        }
    }
}

// 执行更新操作的函数
#[allow(clippy::unused_async)]
#[allow(clippy::module_name_repetitions)]
pub async fn check_update() -> Result<()> {
    println!("{}", "🚀 Starting update process...".bright_blue().bold());

    let updater = self_update::backends::github::Update::configure()
        .repo_owner("zhuima")
        .repo_name("Whoamifuck")
        .bin_name("whoamifuck")
        .current_version(cargo_crate_version!())
        .build()?;

    // 首先检查是否有新版本
    match updater.get_latest_release() {
        Ok(latest) => {
            let current_version = Version::parse(cargo_crate_version!())?;
            let latest_version = Version::parse(&latest.version)?;

            if latest_version <= current_version {
                println!(
                    "{}",
                    format!("✓ Already up to date. Current version: {current_version}")
                        .bright_green()
                        .bold()
                );
                return Ok(());
            }

            // 有新版本，执行更新
            println!(
                "{}",
                format!(
                    "Found new version: {} -> {}",
                    current_version.to_string().yellow(),
                    latest_version.to_string().bright_green()
                )
                .bold()
            );
            println!("{}", "Starting download...".bright_blue().bold());

            // 执行更新
            let status = match updater.update() {
                Ok(status) => status,
                Err(e) => {
                    eprintln!("Update failed: {e}");
                    if e.to_string().contains("permission denied") {
                        eprintln!("Try running with sudo or administrator privileges.");
                    }
                    return Err(anyhow::anyhow!(e));
                }
            };

            if status.updated() {
                println!("\n{}", "✨ Update successful! ✨".bright_green().bold());
                println!(
                    "{}",
                    "┌─────────────────────────────────┐".bright_blue().bold()
                );
                println!(
                    "{} {}",
                    "│ Updated to version:".bright_blue().bold(),
                    status.version().bright_green().bold()
                );
                println!(
                    "{}",
                    "└─────────────────────────────────┘".bright_blue().bold()
                );
            } else {
                println!(
                    "{}",
                    "⚠ Update process completed but version remains unchanged."
                        .bright_yellow()
                        .bold()
                );
            }
        }
        Err(e) => {
            eprintln!("Failed to check latest version: {e}");
            if e.to_string().contains("timeout") {
                eprintln!("Network timeout. Please check your internet connection.");
            } else if e.to_string().contains("rate limit") {
                eprintln!("GitHub API rate limit exceeded. Please try again later.");
            }
            return Err(anyhow::anyhow!(e));
        }
    }

    Ok(())
}
