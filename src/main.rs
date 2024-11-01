#![warn(clippy::all, clippy::pedantic)]
use clap::{Parser, Subcommand};
use commands::{
    complete::Complete, misc::Misc, output::Output, quick::Quick, risk::Risk, special::Special,
};
use std::env;
use std::process;
use utils::banner::{get_banner, VERSION_INFO};

mod commands;
mod update;
mod utils;

#[derive(Parser)]
#[command(
    name = "Whoamifuck",
    author = env!("CARGO_PKG_AUTHORS"),
    version = VERSION_INFO,
    about = "A Rust-based system security analysis and assessment.",
    long_about = None,
    before_help = get_banner()
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Quick command for basic operations")]
    Quick(Quick),
    #[command(about = "Special command for advanced operations")]
    Special(Special),
    #[command(about = "Risk assessment command")]
    Risk(Risk),
    #[command(about = "Miscellaneous command for various tasks")]
    Misc(Misc),
    #[command(about = "Output command for generating reports")]
    Output(Output),
    #[command(about = "Generate shell completion scripts")]
    Complete(Complete),
    #[command(
        about = "Check and update to the latest version",
        long_about = "Automatically check and update Whoamifuck to the latest version from GitHub releases"
    )]
    Update,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // 检查是否是在执行补全相关的命令
    let args: Vec<String> = std::env::args().collect();
    #[allow(clippy::single_char_pattern)]
    let is_completion = args.iter().any(|arg| {
        arg.contains("complete") || 
        arg.contains("completion") || 
        arg.starts_with("_") ||  // bash completion 函数通常以下划线开头
        std::env::var("COMP_LINE").is_ok() || // bash completion 环境变量
        std::env::var("COMP_POINT").is_ok() // bash completion 环境变量
    });

    // 只在非补全命令且是交互式终端时检查更新
    if !is_completion
        && !matches!(cli.command, Some(Commands::Complete(_)))
        && atty::is(atty::Stream::Stdout)
    {
        let _ = update::check_version().await;
    }

    match cli.command {
        Some(command) => {
            // 获取原始参数
            let matches = std::env::args().collect::<Vec<_>>();
            // 第一个参数是程序名，第二个是子命令名，所以如果长度<=2就表示没有额外参数
            // 不在补全模式下才显示帮助
            if matches.len() <= 2 && !matches!(command, Commands::Update) && !is_completion {
                // 获取子命令名称并显示对应的帮助信息
                let subcommand = matches.get(1).map_or("", |s| s.as_str());
                Cli::parse_from([env!("CARGO_PKG_NAME"), subcommand, "--help"]);
                process::exit(0);
            }

            match command {
                Commands::Quick(quick) => {
                    if let Err(e) = quick.run() {
                        eprintln!("Error: {e}");
                        process::exit(1);
                    }
                }
                Commands::Special(special) => {
                    if let Err(e) = special.run() {
                        eprintln!("Error: {e}");
                        process::exit(1);
                    }
                }
                Commands::Risk(risk) => {
                    if let Err(e) = risk.run() {
                        eprintln!("Error: {e}");
                        process::exit(1);
                    }
                }
                Commands::Misc(misc) => {
                    if let Err(e) = misc.run().await {
                        eprintln!("Error: {e}");
                        process::exit(1);
                    }
                }
                Commands::Output(output) => {
                    if let Err(e) = output.run() {
                        eprintln!("Error: {e}");
                        process::exit(1);
                    }
                }
                Commands::Complete(complete) => {
                    if let Err(e) = complete.run() {
                        eprintln!("Error: {e}");
                        process::exit(1);
                    }
                }
                Commands::Update => {
                    if let Err(e) = update::check_update().await {
                        eprintln!("Error: {e}");
                        process::exit(1);
                    }
                }
            }
        }
        None => {
            if !is_completion {
                Cli::parse_from(["whoamifuck", "--help"]);
                process::exit(0);
            }
        }
    }
    Ok(())
}
