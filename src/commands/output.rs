use askama::Template;
use chrono::{FixedOffset, Local, Utc};
use clap::Parser;
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use sysinfo::{CpuExt, ProcessExt, System, SystemExt};

use crate::commands::{quick::Quick, special::Special};

#[derive(Template)]
#[template(path = "report.html")]
struct ReportTemplate {
    timestamp: String,
    system_info: SystemInfo,
    process_info: String,
    network_info: String,
    user_info: String,
    history_info: String,
    crontab_info: String,
    file_info: String,
}

struct SystemInfo {
    hostname: String,
    os_version: String,
    kernel_version: String,
    cpu_usage: f32,
    memory_usage: f32,
    disk_usage: String,
}

#[derive(Parser, Debug)]
#[command(name="output", author, version, about="", long_about = None)]
pub struct Output {
    #[arg(short, long, help = "output to file", default_value_t = false)]
    pub output: bool,

    #[arg(short, long, help = "output to terminal", default_value_t = false)]
    pub html: bool,
}

#[allow(clippy::unused_self)]
#[allow(dead_code)]
impl Output {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.output {
            self.fk_output()?;
        }

        if self.html {
            self.generate_html_report()?;
        }
        Ok(())
    }

    fn fk_output(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "=".repeat(50));
        println!("Generating Output Report:");
        println!("{}", "=".repeat(50));

        // 获取当前目录作为基础目录
        let current_dir = env::current_dir()?;
        println!("Working directory: {}", current_dir.display());

        // 创建输出目录结构
        let output_dir = current_dir.join("output");
        let output_text_dir = output_dir.join("text");
        println!("Creating output directories:");
        println!("  - Main output dir: {}", output_dir.display());
        println!("  - Text output dir: {}", output_text_dir.display());

        fs::create_dir_all(&output_dir)?;
        fs::create_dir_all(&output_text_dir)?;

        // 获取当前时间戳（使用本地时间）
        let local_time = Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap());
        let current_time = local_time.format("%Y%m%d_%H%M%S").to_string();

        // 收集各种信息
        Self::collect_login_info(&output_dir)?;
        Self::collect_history(&output_dir)?;
        Self::collect_crontab_info(&output_dir)?;
        Self::collect_binary_hashes(&output_dir)?;
        Self::collect_ssh_keys(&output_dir)?;

        // 打包所有输出文件
        let archive_name = format!("whoamifuck_report_{current_time}.tar.gz");
        let archive_path = output_text_dir.join(&archive_name);
        println!("\nCreating archive:");
        println!("  - Archive name: {archive_name}");
        println!("  - Archive path: {}", archive_path.display());

        // 修改压缩命令
        Command::new("tar")
            .current_dir(&output_dir) // 设置工作目录
            .args([
                "czf",                          // 创建gzip压缩文件
                archive_path.to_str().unwrap(), // 输出文件路径
                "chief_*",                      // 要压缩的文件
            ])
            .output()?;

        // 清理临时文件
        for entry in fs::read_dir(&output_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|s| s.starts_with("chief_"))
            {
                fs::remove_file(path)?;
            }
        }

        println!("\nReport generation completed:");
        println!("  - Archive location: {}", archive_path.display());
        println!("  - Report directory: {}", output_text_dir.display());
        println!("  - Working directory: {}", current_dir.display());

        Ok(())
    }

    fn collect_login_info(output_dir: &Path) -> io::Result<()> {
        let userinfo_all_path = output_dir.join("chief_userlogin_info_all.txt");

        // 检查系统类型并读取相应的日志文件
        let log_file = if Command::new("which")
            .arg("apt")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
        {
            "/var/log/auth.log"
        } else {
            "/var/log/secure"
        };

        if let Ok(content) = fs::read_to_string(log_file) {
            let last_lines: Vec<&str> = content.lines().rev().take(20000).collect();
            let mut all_file = File::create(&userinfo_all_path)?;
            for line in last_lines.iter().rev() {
                writeln!(all_file, "{line}")?;
            }
        }

        Ok(())
    }

    fn collect_history(output_dir: &Path) -> io::Result<()> {
        let history_all_path = output_dir.join("chief_history_allusers.txt");
        let mut history_file = File::create(history_all_path)?;

        // 收集所有用户的历史命令
        if let Ok(entries) = fs::read_dir("/home") {
            for entry in entries.filter_map(Result::ok) {
                let user_dir = entry.path();
                if user_dir.is_dir() {
                    let history_files = [
                        ".bash_history",
                        ".zsh_history",
                        ".csh_history",
                        ".tcsh_history",
                        ".fish_history",
                    ];

                    for hist_file in &history_files {
                        let hist_path = user_dir.join(hist_file);
                        if hist_path.exists() {
                            writeln!(
                                history_file,
                                "-------------| {} history | ----------------",
                                user_dir.display()
                            )?;
                            if let Ok(content) = fs::read_to_string(&hist_path) {
                                write!(history_file, "{content}")?;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn collect_crontab_info(output_dir: &Path) -> io::Result<()> {
        let cron_spool_path = output_dir.join("chief_cron_spool.txt");
        let crond_path = output_dir.join("chief_crond.txt");

        // 收集 /var/spool/cron 下的文件
        if let Ok(entries) = fs::read_dir("/var/spool/cron") {
            let mut spool_file = File::create(&cron_spool_path)?;
            for entry in entries.filter_map(Result::ok) {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    write!(spool_file, "{content}")?;
                }
            }
        }

        // 收集 /etc/cron.* 下的文件
        let mut crond_file = File::create(&crond_path)?;
        for dir in &["daily", "weekly", "monthly", "hourly"] {
            let cron_dir = format!("/etc/cron.{dir}");
            if let Ok(entries) = fs::read_dir(&cron_dir) {
                for entry in entries.filter_map(Result::ok) {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        write!(crond_file, "{content}")?;
                    }
                }
            }
        }

        Ok(())
    }

    fn collect_binary_hashes(output_dir: &Path) -> io::Result<()> {
        let hash_file_path = output_dir.join("chief_binhashfile.txt");
        let mut hash_file = File::create(hash_file_path)?;

        for dir in &["/usr/bin", "/usr/local/bin", "/bin"] {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.filter_map(Result::ok) {
                    if entry.path().is_file() {
                        if let Ok(output) = Command::new("md5sum").arg(entry.path()).output() {
                            write!(hash_file, "{}", String::from_utf8_lossy(&output.stdout))?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn collect_ssh_keys(output_dir: &Path) -> io::Result<()> {
        let ssh_key_path = output_dir.join("chief_sshpublickey.txt");
        let home = env::var("HOME").unwrap_or_else(|_| String::from("/root"));
        let auth_keys_path = PathBuf::from(home).join(".ssh/authorized_keys");

        if auth_keys_path.exists() {
            fs::copy(auth_keys_path, ssh_key_path)?;
        }

        Ok(())
    }

    fn generate_html_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 创建输出目录
        let output_dir = PathBuf::from("output/html");
        fs::create_dir_all(&output_dir)?;

        // 收集系统信息
        let system_info = self.collect_system_info()?;

        // 收集其他信息
        let process_info = self.collect_process_info()?;
        let network_info = self.collect_network_info()?;
        let user_info = self.collect_user_info()?;
        let history_info = self.collect_history_info()?;
        let crontab_info = self.collect_crontab_html_info()?;
        let file_info = self.collect_file_info()?;

        // 生成报告
        let template = ReportTemplate {
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            system_info,
            process_info,
            network_info,
            user_info,
            history_info,
            crontab_info,
            file_info,
        };

        // 渲染模板
        let html = template.render()?;

        // 保存报告
        let report_path = output_dir.join(format!(
            "report_{}.html",
            Local::now().format("%Y%m%d_%H%M%S")
        ));
        fs::write(&report_path, html)?;

        println!("HTML report generated: {}", report_path.display());
        Ok(())
    }

    fn collect_system_info(&self) -> Result<SystemInfo, Box<dyn std::error::Error>> {
        let mut sys = System::new_all();
        sys.refresh_all();

        Ok(SystemInfo {
            hostname: sys.host_name().unwrap_or_default(),
            os_version: sys.os_version().unwrap_or_default(),
            kernel_version: sys.kernel_version().unwrap_or_default(),
            cpu_usage: sys.cpus().iter().map(CpuExt::cpu_usage).sum::<f32>()
                / sys.cpus().len() as f32,
            memory_usage: (sys.used_memory() as f64 / sys.total_memory() as f64 * 100.0) as f32,
            disk_usage: String::new(), // 使用 df 命令获取
        })
    }

    fn collect_process_info(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut output = String::new();
        let mut sys = System::new_all();
        sys.refresh_all();

        for (pid, process) in sys.processes() {
            output.push_str(&format!(
                "{}\t{}\t{:.2}%\n",
                pid,
                process.name(),
                process.cpu_usage()
            ));
        }
        Ok(output)
    }

    fn collect_network_info(&self) -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new("netstat").args(["-tuln"]).output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn collect_user_info(&self) -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new("cat").arg("/etc/passwd").output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn collect_history_info(&self) -> Result<String, Box<dyn std::error::Error>> {
        let home = env::var("HOME")?;
        let history_path = PathBuf::from(home).join(".bash_history");
        Ok(fs::read_to_string(history_path)?)
    }

    fn collect_crontab_html_info(&self) -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new("crontab").arg("-l").output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn collect_file_info(&self) -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new("find")
            .args(["/", "-type", "f", "-mtime", "-3"])
            .output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
