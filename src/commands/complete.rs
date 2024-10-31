use clap::{CommandFactory, Parser};
use clap_complete::generate;
#[allow(clippy::wildcard_imports)]
use clap_complete::shells::*;
use std::io::stdout;

#[derive(Parser, Debug)]
#[command(name = "complete", about = "Generate shell completion scripts")]
pub struct Complete {
    #[arg(
        value_enum,
        help = "Shell to generate completion for (bash, zsh, fish, powershell, elvish)"
    )]
    pub shell: Shell,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Powershell,
    Elvish,
}

#[allow(clippy::unnecessary_wraps)]
impl Complete {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = crate::Cli::command();
        match self.shell {
            Shell::Bash => {
                generate(Bash, &mut cmd, "whoamifuck", &mut stdout());
                eprintln!("\nTo enable completions, run:\n");
                eprintln!("# Install bash-completion if not already installed:");
                eprintln!("# For Debian/Ubuntu:");
                eprintln!("apt-get install bash-completion");
                eprintln!("# For CentOS/RHEL:");
                eprintln!("yum install bash-completion");
                eprintln!("\n# Add to your ~/.bashrc:");
                eprintln!("source <(whoamifuck complete bash)");
            }
            Shell::Zsh => {
                generate(Zsh, &mut cmd, "whoamifuck", &mut stdout());
                eprintln!("\nTo enable completions, run:\n");
                eprintln!("# Add to your ~/.zshrc:");
                eprintln!("source <(whoamifuck complete zsh)");
            }
            Shell::Fish => {
                generate(Fish, &mut cmd, "whoamifuck", &mut stdout());
                eprintln!("\nTo enable completions, run:\n");
                eprintln!("# Fish will automatically load completions from:");
                eprintln!("whoamifuck complete fish > ~/.config/fish/completions/whoamifuck.fish");
            }
            Shell::Powershell => {
                generate(PowerShell, &mut cmd, "whoamifuck", &mut stdout());
                eprintln!("\nTo enable completions, run:\n");
                eprintln!("# Add to your PowerShell profile:");
                eprintln!("whoamifuck complete powershell | Out-String | Invoke-Expression");
            }
            Shell::Elvish => {
                generate(Elvish, &mut cmd, "whoamifuck", &mut stdout());
                eprintln!("\nTo enable completions, run:\n");
                eprintln!("# Add to your ~/.elvish/rc.elv:");
                eprintln!("eval (whoamifuck complete elvish | slurp)");
            }
        }
        Ok(())
    }
}
