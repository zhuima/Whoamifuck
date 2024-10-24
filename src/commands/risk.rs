use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="risk", author, version, about="", long_about = None)]
pub struct Risk {
    #[arg(short, long, help = "security baseline check")]
    pub baseline: String,

    #[arg(short, long, help = "check system vulnerability information")]
    pub risks: String,

    #[arg(short, long, help = "check system rootkit information")]
    pub rootkitcheck: String,

    #[arg(
        short,
        long,
        help = "check web shell information",
        default_value = "[default:/var/www/;/www/wwwroot/..]"
    )]
    pub webshell: String,
}
