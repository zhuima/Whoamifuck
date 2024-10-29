use clap::Parser;
use sysinfo::{NetworkExt, ProcessExt, System, SystemExt};

#[derive(Parser, Debug)]
#[command(name="special", author, version, about="", long_about = None)]
pub struct Special {
    #[arg(short = 'x', long, help = "check user process and service status")]
    pub proc_serv: bool,

    #[arg(short, long, help = "check user port open status")]
    pub port: bool,

    #[arg(short, long, help = "check system status information")]
    pub os_status: bool,
}

impl Special {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut system = System::new_all();
        system.refresh_all();

        if self.proc_serv {
            self.fk_procserv(&system)?;
        }
        if self.port {
            self.fk_portstatus(&system)?;
        }
        if self.os_status {
            self.check_system_status(&system)?;
        }
        Ok(())
    }

    #[allow(clippy::unused_self, clippy::unnecessary_wraps)]
    fn fk_procserv(&self, system: &System) -> Result<(), Box<dyn std::error::Error>> {
        println!("Process Information:");
        for (pid, process) in system.processes() {
            println!("{}\t{}\t{}", pid, process.name(), process.cpu_usage());
        }

        println!("\n{}", "=".repeat(50));
        println!("Running Services:");
        // Note: sysinfo doesn't provide direct access to services
        // You might need to use a platform-specific approach or another library for this
        println!("Service information not available through sysinfo");

        Ok(())
    }

    #[allow(clippy::unused_self, clippy::unnecessary_wraps)]
    fn fk_portstatus(&self, system: &System) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "=".repeat(50));
        println!("Port Open Status:");
        println!("{}", "=".repeat(50));

        for (interface_name, data) in system.networks() {
            println!("{interface_name}:");
            println!("  Received: {} B", data.received());
            println!("  Transmitted: {} B", data.transmitted());
        }

        Ok(())
    }

    #[allow(clippy::unused_self, clippy::unnecessary_wraps)]
    fn check_system_status(&self, system: &System) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "=".repeat(50));
        println!("System Status:");
        println!("{}", "=".repeat(50));
        println!("Total memory: {} KB", system.total_memory());
        println!("Used memory: {} KB", system.used_memory());
        println!("Total swap: {} KB", system.total_swap());
        println!("Used swap: {} KB", system.used_swap());
        println!("System name: {:?}", system.name());
        println!("System kernel version: {:?}", system.kernel_version());
        println!("System OS version: {:?}", system.os_version());
        println!("System host name: {:?}", system.host_name());

        Ok(())
    }
}
