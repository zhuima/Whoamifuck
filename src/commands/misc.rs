use clap::Parser;
use std::path::PathBuf;
use std::str::FromStr;
use url::Url;

#[derive(Parser, Debug, Clone)]
#[command(name="misc", author, version, about="", long_about = None)]
pub struct Misc {
    #[arg(short, long, help = "check page live status (URL or file path)")]
    pub code: FileOrUrl,

    #[arg(short, long, help = "check user information")]
    pub sqletlog: PathBuf,

    #[arg(short, long, help = "set crontab information")]
    pub auto_run: String,

    #[arg(
        short,
        long,
        help = "custom command define test",
        default_value = "[default:~/.whok/chief-inspector.conf]"
    )]
    pub ext: PathBuf,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum FileOrUrl {
    File(PathBuf),
    Url(Url),
}

impl FromStr for FileOrUrl {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(url) = Url::parse(s) {
            Ok(FileOrUrl::Url(url))
        } else {
            Ok(FileOrUrl::File(PathBuf::from(s)))
        }
    }
}
