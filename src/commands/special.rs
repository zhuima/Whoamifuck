use byte_unit::{Byte, UnitType};
use clap::Parser;
use mac_address::get_mac_address;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::process::Command;
use std::time::Duration;
use sysinfo::{CpuExt, DiskExt, ProcessExt, System, SystemExt};

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
            println!("{}\t{}\t{:.2}%", pid, process.name(), process.cpu_usage());
        }

        println!("\n{}", "=".repeat(50));
        println!("Running Services:");

        // 使用 systemctl 命令获取服务信息
        let output = Command::new("systemctl")
            .args([
                "list-units",
                "--type=service",
                "--state=running",
                "--no-pager",
                "--no-legend",
            ])
            .output()?;

        if output.status.success() {
            let services = String::from_utf8_lossy(&output.stdout);
            println!("{:<40} {:<40}", "Service Name", "Description");
            println!("{}", "=".repeat(80));
            for line in services.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let service_name = parts[0].trim_end_matches(".service");
                    let description = parts[3..].join(" ");
                    println!("{service_name:<40} {description:<40}");
                }
            }
        } else {
            println!("Failed to retrieve service information");
        }

        Ok(())
    }

    #[allow(clippy::unused_self, clippy::unnecessary_wraps)]
    fn fk_portstatus(&self, _system: &System) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "=".repeat(50));
        println!("Port Open Status:");
        println!("{}", "=".repeat(50));

        let localhost = Ipv4Addr::new(127, 0, 0, 1);
        let port_range = 1..65535; // 扫描从1到65535的所有端口
        let timeout = Duration::from_millis(100); // 设置100毫秒的超时

        #[allow(clippy::single_match)]
        for port in port_range {
            let socket = SocketAddrV4::new(localhost, port);
            match TcpStream::connect_timeout(&socket.into(), timeout) {
                Ok(_) => println!("Port {port} is open"),
                Err(_) => {}
            }
        }

        Ok(())
    }

    #[allow(clippy::unused_self, clippy::unnecessary_wraps)]
    fn check_system_status(&self, system: &System) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "=".repeat(50));
        println!("System Status:");
        println!("{}", "=".repeat(50));

        // Memory information
        let total_memory = Byte::from_u64(system.total_memory());
        let used_memory = Byte::from_u64(system.used_memory());
        let total_swap = Byte::from_u64(system.total_swap());
        let used_swap = Byte::from_u64(system.used_swap());

        println!("Memory Information:");
        println!(
            "  Total memory: {}",
            total_memory.get_appropriate_unit(UnitType::Decimal)
        );
        println!(
            "  Used memory: {}",
            used_memory.get_appropriate_unit(UnitType::Decimal)
        );
        println!(
            "  Total swap: {}",
            total_swap.get_appropriate_unit(UnitType::Decimal)
        );
        println!(
            "  Used swap: {}",
            used_swap.get_appropriate_unit(UnitType::Decimal)
        );

        // CPU information
        println!("\nCPU Information:");
        for (i, cpu) in system.cpus().iter().enumerate() {
            println!("  CPU {}: {}% used", i, cpu.cpu_usage());
        }

        // Disk information
        println!("\nDisk Information:");
        for disk in system.disks() {
            let total_space = Byte::from_u64(disk.total_space());
            let available_space = Byte::from_u64(disk.available_space());
            println!("  {}: ", disk.name().to_string_lossy());
            println!(
                "    Total: {}",
                total_space.get_appropriate_unit(UnitType::Decimal)
            );
            println!(
                "    Available: {}",
                available_space.get_appropriate_unit(UnitType::Decimal)
            );
            println!("    Is removable: {}", disk.is_removable());
        }

        // System information
        println!("\nSystem Information:");
        println!("  System name: {:?}", system.name());
        println!("  System kernel version: {:?}", system.kernel_version());
        println!("  System OS version: {:?}", system.os_version());
        println!("  System host name: {:?}", system.host_name());

        match get_mac_address() {
            Ok(Some(addr)) => println!("  MAC Address: {addr}"),
            Ok(None) => println!("  MAC Address: No MAC address found"),
            Err(e) => println!("  MAC Address: Error getting MAC address: {e:?}"),
        }

        Ok(())
    }
}
