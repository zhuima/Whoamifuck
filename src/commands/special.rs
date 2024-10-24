use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="special", author, version, about="", long_about = None)]
pub struct Special {
    #[arg(short, long, help = "check user process and service status")]
    pub proc_serv: String,

    #[arg(short, long, help = "check user port open status")]
    pub port: i32,

    #[arg(short, long, help = "check system status information")]
    pub os_status: String,
}
