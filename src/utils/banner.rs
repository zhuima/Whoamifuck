use colored::Colorize;

pub const VERSION_INFO: &str = concat!(
    "Version: ",
    env!("CARGO_PKG_VERSION"),
    "\n",
    "Git Hash: ",
    env!("GIT_HASH"),
    "\n",
    "Git Branch: ",
    env!("GIT_BRANCH"),
    "\n",
    "Git Tag: ",
    env!("GIT_TAG"),
    "\n",
    "Build Time: ",
    env!("BUILD_TIME"),
    "\n",
    "Rust Version: ",
    env!("RUST_VERSION"),
    "\n",
    "Cargo Version: ",
    env!("CARGO_VERSION")
);

#[allow(clippy::format_in_format_args)]
#[allow(clippy::module_name_repetitions)]
pub fn get_banner() -> String {
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
