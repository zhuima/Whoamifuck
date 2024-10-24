use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="quick", author, version, about="", long_about = None)]
pub struct Quick {
    #[arg(short, long, help = "The device name of the user")]
    pub user_device: String,

    #[arg(
        short,
        long,
        help = "The login name of the user",
        default_value = "[default:/var/log/secure;/var/log/auth.log]"
    )]
    pub login: String,

    #[arg(short, long, help = "basic output")]
    pub nomal: bool,

    #[arg(short, long, help = "full output")]
    pub all: bool,
}
