#![warn(clippy::all, clippy::pedantic)]
use clap::{Parser, Subcommand};
use commands::{misc::Misc, output::Output, quick::Quick, risk::Risk, special::Special};
use std::env;
use std::process;

mod commands;
mod utils; // 导入 utils 模块

const VERSION_INFO: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (commit: ",
    env!("GIT_HASH"),
    ", built on: ",
    env!("BUILD_DATE"),
    ")"
);

const BANNER: &str = r"
__        __   _                            _____ _    _  ______ _  __
\ \      / /__| | ___ ___  _ __ ___   ___  |  ___| |  | |  ____| |/ /
 \ \ /\ / / _ \ |/ __/ _ \| '_ ` _ \ / _ \ | |_  | |  | | |__  | ' / 
  \ V  V /  __/ | (_| (_) | | | | | |  __/ |  _| | |__| |  __| | . \ 
   \_/\_/ \___|_|\___\___/|_| |_| |_|\___| |_| who! \____/|_|    |_|\_\
                                                                     
";

#[derive(Parser)]
#[command(
    name = "Whoamifuck",
    author = env!("CARGO_PKG_AUTHORS"),
    version = VERSION_INFO,
    about = "Whoamifuck，zhuima first open source tool. This is a tool written in Rust to detect intruders, after the function update, is not limited to checking users' login information.",
    long_about = None,
    before_help = BANNER
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
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
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    #[allow(clippy::single_match_else)]
    match cli.command {
        Some(command) => match command {
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
            Commands::Risk(risk) => println!("RISK: {risk:?}"),
            Commands::Misc(misc) => println!("MISC: {misc:?}"),
            Commands::Output(output) => println!("OUTPUT: {output:?}"),
        },
        None => {
            // 打印帮助信息
            Cli::parse_from(["Whoamifuck", "--help"]);
            // 正常退出，退出码为 0
            process::exit(0);
        }
    }
}
