use std::fs;
use std::process::Command;

pub fn fk_baseinfo(device: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "=".repeat(50));
    println!("Basic System Information:");
    println!("{}", "=".repeat(50));

    // 获取主机名
    let output = Command::new("hostname")
        .output()
        .map_err(|e| format!("Failed to get hostname: {e}"))?;
    if output.status.success() {
        println!(
            "Hostname: {}",
            String::from_utf8_lossy(&output.stdout).trim()
        );
    }

    // 获取系统信息
    if let Ok(output) = Command::new("uname").arg("-a").output() {
        println!("System: {}", String::from_utf8_lossy(&output.stdout).trim());
    }

    // 获取网络信息
    if let Some(dev) = device {
        // 如果指定了设备，只显示该设备的信息
        let output = Command::new("ip")
            .args(["addr", "show", dev])
            .output()
            .map_err(|e| format!("Failed to get network info: {e}"))?;
        if output.status.success() {
            println!("\nNetwork Device {dev}:");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
    } else {
        // 否则显示所有网络设备的信息
        let output = Command::new("ip")
            .args(["addr"])
            .output()
            .map_err(|e| format!("Failed to get network info: {e}"))?;
        if output.status.success() {
            println!("\nNetwork Devices:");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }

    // 获取 DNS 信息
    if let Ok(content) = fs::read_to_string("/etc/resolv.conf") {
        println!("\nDNS Configuration:");
        for line in content.lines() {
            if line.starts_with("nameserver") {
                println!("{line}");
            }
        }
    }

    // 获取默认网关
    let output = Command::new("ip")
        .args(["route", "show", "default"])
        .output()
        .map_err(|e| format!("Failed to get default route: {e}"))?;
    if output.status.success() {
        println!("\nDefault Gateway:");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    println!();
    Ok(())
}
