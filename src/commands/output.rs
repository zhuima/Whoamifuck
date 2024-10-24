use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="output", author, version, about="", long_about = None)]
pub struct Output {
    #[arg(short, long, help = "output to file")]
    pub output: String,

    #[arg(short, long, help = "output to terminal")]
    pub html: bool,
}
