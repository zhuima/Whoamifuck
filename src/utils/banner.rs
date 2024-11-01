use colored::Colorize;

const BANNER_WIDTH: usize = 60;

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
        "{}\n{}\n{}\n{}\n{}",
        "_".repeat(BANNER_WIDTH),
        format!(
            ": {:<50}",
            "代码仓库地址: https://github.com/zhuima/whoamifuck"
        ),
        format!(": {:<50}", "小报童甄选专栏: https://xiaobaot.best"),
        format!(
            ": {:<50}",
            "A Rust-based system security analysis and assessment"
        ),
        "-".repeat(BANNER_WIDTH)
    );

    format!("\n{banner}\n{}", info.yellow().bold())
}
