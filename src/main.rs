use std::path::PathBuf;
use url::Url;

use clap::{Parser, Subcommand, ValueEnum};
use tokio;





#[derive(Parser)]
#[command(name="Whoamifuck", author, version, about="Whoamifuck，Eonian sharp's first open source tool. This is a tool written by shell to detect intruders, after the function update, is not limited to checking users' login information.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long)]
    version: String,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Quick command for basic operations")]
    QUICK(Quick),
    #[command(about = "Special command for advanced operations")]
    SPECIAL(Special),
    #[command(about = "Risk assessment command")]
    RISK(Risk),
    #[command(about = "Miscellaneous command for various tasks")]
    MISC(Misc),
    #[command(about = "Output command for generating reports")]
    OUTPUT(Output),
}


#[derive(Debug, Clone)]
enum FileOrUrl {
    File(PathBuf),
    Url(Url),
}


#[derive(Parser, Debug)]
#[command(name="quick", author, version, about="", long_about = None)]
struct Quick {
    // Add fields specific to QUICK command
    #[arg(short, long, help = "The device name of the user")]
    user_device: String,

    #[arg(short, long, help = "The login name of the user", default_value = "[default:/var/log/secure;/var/log/auth.log]")]
    login: String,

    #[arg(short, long, help = "basic output")]
    nomal: bool,

    #[arg(short, long, help = "full output")]
    all: bool,
}

#[derive(Parser, Debug)]
#[command(name="special", author, version, about="", long_about = None)]
struct Special {
    // Add fields specific to SPECIAL command
    #[arg(short, long, help = "check user process and service status")]
    proc_serv: String,

    #[arg(short, long, help = "check user port open status")]
    port: i32,

    #[arg(short, long, help = "check system status information")]
    os_status: String,

}

#[derive(Parser, Debug)]
#[command(name="risk", author, version, about="", long_about = None)]
struct Risk {
    // Add fields specific to RISK command
    #[arg(short, long, help = "security baseline check")]
    baseline: String,

    #[arg(short, long, help = "check system vulnerability information")]
    risk: String,

    #[arg(short, long, help = "check system rootkit information")]
    rootkitcheck: String,


    #[arg(short, long, help = "check web shell information", default_value = "[default:/var/www/;/www/wwwroot/..]")]
    webshell: String,

}

#[derive(Parser, Debug, Clone)]
#[command(name="misc", author, version, about="", long_about = None)]
struct Misc {
    // Add fields specific to MISC command
    #[arg(short, long, help = "check page live status")]
    code: Option<FileOrUrl>,

    #[arg(short, long, help = "check user information")]
    sqletlog: PathBuf,

    #[arg(short, long, help = "set crontab information")]
    auto_run: String,

    #[arg(short, long, help = "custom command define test", default_value = "[default:~/.whok/chief-inspector.conf]")]
    ext: std::path::PathBuf,

}

#[derive(Parser, Debug)]
#[command(name="output", author, version, about="", long_about = None)]
struct Output {
    // Add fields specific to OUTPUT command
    #[arg(short, long, help = "output to file")]
    output: String,

    #[arg(short, long, help = "output to terminal")]
    html: bool,
}

#[tokio::main]
async fn main() {
    let mut cli = Cli::parse();

    cli.version = "1.0.0".to_string();

    
    match &cli.command {
        Commands::QUICK(quick) => println!("QUICK: {:?}", quick),
        Commands::SPECIAL(special) => println!("SPECIAL: {:?}", special),
        Commands::RISK(risk) => println!("RISK: {:?}", risk),
        Commands::MISC(misc) => println!("MISC: {:?}", misc),
        Commands::OUTPUT(output) => println!("OUTPUT: {:?}", output),
    }
}
