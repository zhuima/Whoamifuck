#![warn(clippy::all, clippy::pedantic)]
use clap::{Parser, Subcommand};
use colored::Colorize;
use commands::{misc::Misc, output::Output, quick::Quick, risk::Risk, special::Special};
use std::env;
use std::process;

mod commands;
mod utils;

const VERSION_INFO: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (commit: ",
    env!("GIT_HASH"),
    ", built on: ",
    env!("BUILD_DATE"),
    ")"
);

#[allow(clippy::format_in_format_args)]
fn get_banner() -> String {
    let banner = format!(
        "\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        " ██╗    ██╗██╗  ██╗ ██████╗  █████╗ ███╗   ███╗██╗    ███████╗██╗   ██╗ ██████╗██╗  ██╗"
            .bright_red(),
        " ██║root██║██║  ██║██╔═══██╗██╔══██╗████╗ ████║██║    ██╔════╝██║   ██║██╔════╝██║ ██╔╝"
            .bright_red(),
        " ██║ █╗ ██║███████║██║777██║███████║██╔████╔██║██║    █████╗  ██║   ██║██║<bug>█████╔╝ "
            .bright_red(),
        " ██║███╗██║██╔══██║██║   ██║██╔══██║██║╚██╔╝██║██║    ██╔══╝  ██║   ██║██║     ██╔═██╗ "
            .bright_red(),
        " ╚███╔███╔╝██║  ██║╚██████╔╝██║  ██║██║ ╚═╝ ██║██║    ██║     ╚██████╔╝╚██████╗██║  ██╗"
            .bright_red(),
        "  ╚══╝╚══╝ ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚═╝    ╚═╝ who! ╚═════╝  ╚═════╝╚═╝  ╚═╝"
            .bright_red(),
        format!(
            "       Hi whoamifuck          v{}                by  - {}",
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS").bright_blue()
        )
    );

    let info = format!(
        "{}\n{}\n{}\n{}",
        r"________________________________________________________",
        r": https://github.com/zhuima/whoamifuck                  :",
        r": A Rust-based system security analysis and assessment  :",
        r" ------------------------------------------------------"
    );

    format!("\n{banner}\n{}", info.yellow().bold())
}

#[derive(Parser)]
#[command(
    name = "Whoamifuck",
    author = env!("CARGO_PKG_AUTHORS"),
    version = VERSION_INFO,
    about = "Whoamifuck，zhuima first open source tool. This is a tool written in Rust to detect intruders, after the function update, is not limited to checking users' login information.",
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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
        },
        None => {
            Cli::parse_from(["Whoamifuck", "--help"]);
            process::exit(0);
        }
    }
    Ok(())
}
