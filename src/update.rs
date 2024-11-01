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

    if let Ok(latest) = updater.get_latest_release() {
        // 解析版本号进行比较
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
            return Ok(true);
        } else {
            println!(
                "{}",
                "✓ You are using the latest version!".bright_green().bold()
            );
        }
    } else {
        println!("{}", "✗ Failed to check for updates!".bright_red().bold());
    }
    Ok(false)
}

// 执行更新操作的函数
#[allow(clippy::unused_async)]
#[allow(clippy::module_name_repetitions)]
pub async fn check_update() -> Result<()> {
    println!("{}", "🚀 Starting update process...".bright_blue().bold());

    let status = self_update::backends::github::Update::configure()
        .repo_owner("zhuima")
        .repo_name("Whoamifuck")
        .bin_name("whoamifuck")
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

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
            "ℹ No update available. You are using the latest version."
                .bright_yellow()
                .bold()
        );
    }

    Ok(())
}
