use clap::Parser;
use std::fs::{self, read_dir, File};
use std::io::{BufRead, BufReader};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;
use sysinfo::{CpuExt, System, SystemExt};

// 将常量移到文件顶部
const SECURE_FILE: &str = "/var/log/secure";
const AUTH_LOG_FILE: &str = "/var/log/auth.log";

#[derive(Parser, Debug)]
#[command(name="quick", author, version, about="", long_about = None)]
pub struct Quick {
    #[arg(short, long, help = "The device name of the user")]
    pub user_device: bool,

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

impl Quick {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.user_device {
            Self::fk_baseinfo(None)?;
        }

        if self.nomal {
            Self::fk_baseinfo(None)?;
            Self::fk_history()?;
            Self::fk_crontab()?;
            Self::fk_filemove()?;
            Self::fk_userinfo()?;
        }

        if self.all {
            Self::fk_baseinfo(None)?;
            Self::fk_devicestatus()?;
            Self::fk_userlogin(&self.login)?; // 传递 login 参数
        }

        Ok(())
    }

    fn fk_baseinfo(device: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
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

        // ���取网络信息
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

    fn fk_history() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "=".repeat(50));
        println!("History Information:");
        println!("{}", "=".repeat(50));

        // 获取当前用户的主目录
        let home = std::env::var("HOME")?;
        let home_dir = PathBuf::from(home);

        // 查找所有以 .sh_history 结尾的文件
        let history_files = read_dir(&home_dir)?
            .filter_map(Result::ok)
            .filter(|entry| entry.file_name().to_string_lossy().ends_with("sh_history"));

        // 读取并显示每个历史文件的最后10行
        for entry in history_files {
            let path = entry.path();
            println!("\nHistory from {:?}:", path.file_name().unwrap_or_default());

            match fs::read_to_string(&path) {
                Ok(content) => {
                    let lines: Vec<&str> = content.lines().collect();
                    let start = if lines.len() > 10 {
                        lines.len() - 10
                    } else {
                        0
                    };

                    for line in &lines[start..] {
                        println!("{line}");
                    }
                }
                Err(e) => println!("Error reading history file: {e}"),
            }
        }

        println!();
        Ok(())
    }

    fn fk_crontab() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "=".repeat(50));
        println!("Crontab Information:");
        println!("{}", "=".repeat(50));

        // 检查用户的 crontab
        let output = Command::new("crontab")
            .arg("-l")
            .output()
            .map_err(|e| format!("Failed to execute crontab: {e}"))?;

        if output.status.success() {
            println!("\nUser Crontab:");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }

        // 检查系统 cron 目录
        let cron_dirs = [
            "/var/spool/cron",
            "/etc/cron.d",
            "/etc/cron.daily",
            "/etc/cron.weekly",
            "/etc/cron.hourly",
            "/etc/cron.monthly",
        ];

        for dir in &cron_dirs {
            if let Ok(entries) = fs::read_dir(dir) {
                println!("\nCron tasks in {dir}:");
                println!("{}", "-".repeat(30));

                for entry in entries.filter_map(Result::ok) {
                    let path = entry.path();
                    if path.is_file() {
                        println!("File: {}", path.display());
                        match fs::read_to_string(&path) {
                            Ok(content) => {
                                // 过滤掉空行和注释行
                                let tasks: Vec<&str> = content
                                    .lines()
                                    .filter(|line| {
                                        !line.trim().is_empty() && !line.trim().starts_with('#')
                                    })
                                    .collect();

                                if !tasks.is_empty() {
                                    for task in tasks {
                                        println!("  {task}");
                                    }
                                }
                            }
                            Err(e) => println!("  Error reading file: {e}"),
                        }
                        println!();
                    }
                }
            }
        }

        Ok(())
    }

    fn fk_filemove() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "=".repeat(50));
        println!("File Modification Information:");
        println!("{}", "=".repeat(50));

        // 查找最近三天修改的文件
        println!("\nFiles modified in the last 3 days:");
        println!("{}", "-".repeat(30));
        let output = Command::new("find")
            .args(["-type", "f", "-mtime", "-3"])
            .output()?;
        if output.status.success() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }

        // 查找最近���天创建的文件
        println!("\nFiles created in the last 3 days:");
        println!("{}", "-".repeat(30));
        let output = Command::new("find")
            .args(["-type", "f", "-ctime", "-3"])
            .output()?;
        if output.status.success() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }

        // 查找/var下最近三天修改的文件
        println!("\nFiles modified in /var in the last 3 days:");
        println!("{}", "-".repeat(30));
        let output = Command::new("find")
            .args([
                "/var", "-type", "f", "-mtime", "-3", "-exec", "ls", "-la", "{}", "+",
            ])
            .output()?;
        if output.status.success() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }

        // 检查 SSH authorized_keys 文件
        println!("\nSSH authorized_keys information:");
        println!("{}", "-".repeat(30));
        let home = std::env::var("HOME")?;
        let ssh_key_path = PathBuf::from(home).join(".ssh").join("authorized_keys");

        if ssh_key_path.exists() {
            let metadata = fs::metadata(&ssh_key_path)?;
            let permissions = metadata.permissions();
            let modified = metadata.modified()?;

            // 获取文件权限（Unix样式）
            #[cfg(unix)]
            let mode = permissions.mode() & 0o777;

            #[cfg(unix)]
            println!("Permissions: {mode:o}");
            println!("Last modified: {modified:?}");
        } else {
            println!("SSH authorized_keys file not found");
        }

        println!();
        Ok(())
    }

    fn fk_userinfo() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "=".repeat(50));
        println!("User Information:");
        println!("{}", "=".repeat(50));

        // 读取 /etc/passwd 最新10个用户
        println!("\nLast 10 users from /etc/passwd:");
        println!("{}", "-".repeat(30));
        let file = File::open("/etc/passwd")?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();
        for line in lines.iter().rev().take(10) {
            println!("{line}");
        }

        // 读取 /etc/shadow 最新10个影子
        println!("\nLast 10 entries from /etc/shadow:");
        println!("{}", "-".repeat(30));
        if let Ok(file) = File::open("/etc/shadow") {
            // shadow 文件可能需要 root 权限
            let reader = BufReader::new(file);
            let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();
            for line in lines.iter().rev().take(10) {
                println!("{line}");
            }
        } else {
            println!("Cannot access shadow file (requires root privileges)");
        }

        // 查找具有 root 权限的用户 (UID=0)
        println!("\nUsers with root privileges (UID=0):");
        println!("{}", "-".repeat(30));
        let file = File::open("/etc/passwd")?;
        let reader = BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 && parts[2] == "0" {
                println!("{}", parts[0]);
            }
        }

        // 查找具有远程登录权限的用户
        println!("\nUsers with remote login privileges:");
        println!("{}", "-".repeat(30));
        if let Ok(file) = File::open("/etc/shadow") {
            // shadow 文件可能需要 root 权限
            let reader = BufReader::new(file);
            for line in reader.lines().map_while(Result::ok) {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 2 && !parts[1].starts_with('!') && parts[1] != "*" {
                    println!("{}", parts[0]);
                }
            }
        } else {
            println!("Cannot access shadow file (requires root privileges)");
        }

        // 检查 sudo 权限
        println!("\nUsers with sudo privileges:");
        println!("{}", "-".repeat(30));
        match File::open("/etc/sudoers") {
            Ok(file) => {
                let reader = BufReader::new(file);
                for line in reader.lines().map_while(Result::ok) {
                    let line = line.trim();
                    if !line.starts_with('#') && !line.is_empty() && line.contains("ALL=(ALL)") {
                        println!("{line}");
                    }
                }
            }
            Err(_) => println!("Cannot access sudoers file (requires root privileges)"),
        }

        println!();
        Ok(())
    }

    #[allow(clippy::unnecessary_wraps)]
    #[allow(clippy::cast_precision_loss)]
    fn fk_devicestatus() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "=".repeat(50));
        println!("System Status Information:");
        println!("{}", "=".repeat(50));

        let mut sys = System::new_all();
        sys.refresh_all();

        // 内存使用率
        let total_memory = sys.total_memory() as f64;
        let used_memory = sys.used_memory() as f64;
        let memory_usage = (used_memory / total_memory) * 100.0;
        println!("Memory: {memory_usage:.2}%");

        // 磁盘使用率
        if let Ok(output) = Command::new("df").args(["-h", "/"]).output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = output_str.lines().nth(1) {
                if let Some(usage) = line.split_whitespace().nth(4) {
                    println!("Disk: {usage}");
                }
            }
        }

        // CPU 使用率
        let cpu_usage: f32 =
            sys.cpus().iter().map(CpuExt::cpu_usage).sum::<f32>() / sys.cpus().len() as f32;
        println!("CPU: {cpu_usage:.2}%");

        // 额外的系统信息
        println!("\nAdditional System Information:");
        println!("Total Memory: {} KB", sys.total_memory());
        println!("Used Memory: {} KB", sys.used_memory());
        println!("Total Swap: {} KB", sys.total_swap());
        println!("Used Swap: {} KB", sys.used_swap());

        if let Some(name) = sys.name() {
            println!("OS Name: {name}");
        }
        if let Some(version) = sys.os_version() {
            println!("OS Version: {version}");
        }
        if let Some(kernel) = sys.kernel_version() {
            println!("Kernel Version: {kernel}");
        }

        println!();
        Ok(())
    }

    fn fk_userlogin(login_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "=".repeat(50));
        println!("User Login Information:");
        println!("{}", "=".repeat(50));

        // 修改条件判断的逻辑
        let log_file = if login_file == "[default:/var/log/secure;/var/log/auth.log]" {
            // 如果用户没有指定，则根据系统类型选择默认文件
            let os_release = fs::read_to_string("/etc/os-release")?;
            let os_name = os_release
                .lines()
                .find(|line| line.starts_with("PRETTY_NAME="))
                .and_then(|line| line.split('=').nth(1))
                .unwrap_or("")
                .trim_matches('"');

            if os_name.contains("CentOS") {
                SECURE_FILE
            } else if os_name.contains("Debian")
                || os_name.contains("Ubuntu")
                || os_name.contains("Kali")
                || os_name.contains("Parrot")
                || os_name.contains("Deepin")
            {
                AUTH_LOG_FILE
            } else {
                println!("Unknown distribution, defaulting to RedHat-like system");
                SECURE_FILE
            }
        } else {
            login_file
        };

        // 读取并分析日志文件
        if let Ok(file) = File::open(log_file) {
            let reader = BufReader::new(file);
            println!("\nRecent login attempts from {log_file}:");
            println!("{}", "-".repeat(30));

            for line in reader.lines().map_while(Result::ok) {
                // 使用 map_while 替代 filter_map
                // 根据不同的日志格式进行解析
                if log_file == SECURE_FILE {
                    // CentOS 格式解析
                    if line.contains("Accepted password") || line.contains("Failed password") {
                        println!("{line}");
                    }
                } else {
                    // Debian/Ubuntu 格式解析
                    if line.contains("authentication") || line.contains("session opened") {
                        println!("{line}");
                    }
                }
            }
        } else {
            println!("Log file {log_file} does not exist or is not accessible");
        }

        // 显示当前登录用户
        println!("\nCurrently logged in users:");
        println!("{}", "-".repeat(30));
        if let Ok(output) = Command::new("who").output() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }

        // 显示最近登录记录
        println!("\nRecent login history:");
        println!("{}", "-".repeat(30));
        if let Ok(output) = Command::new("last").args(["-n", "10"]).output() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }

        println!();
        Ok(())
    }
}
