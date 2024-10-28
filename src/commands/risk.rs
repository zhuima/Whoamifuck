use crate::utils::system_utils::fk_baseinfo;
use clap::Parser;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(name="risk", author, version, about="", long_about = None)]
pub struct Risk {
    #[arg(short, long, help = "security baseline check")]
    pub baseline: bool,

    #[arg(short, long, help = "check system vulnerability information")]
    pub risks: bool,

    #[arg(short = 'k', long, help = "check system rootkit information")]
    pub rootkitcheck: bool,

    #[arg(
        short,
        long,
        help = "check web shell information",
        default_value_t = String::from("[default:/var/www/;/www/wwwroot/..]"),
        action = clap::ArgAction::Set,
        num_args = 0..=1
    )]
    pub webshell: String,
}

impl Risk {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.baseline {
            fk_baseinfo(None)?;
        }

        if self.risks {
            Self::fk_vulcheck()?;
        }

        if self.rootkitcheck {
            Self::fk_rookit_analysis()?;
        }

        // 修改判断逻辑
        if self.webshell == "[default:/var/www/;/www/wwwroot/..]" {
            Self::fk_wsfinder(&["/www/wwwroot", "/var/www"])?;
        } else {
            Self::fk_wsfinder(&[&self.webshell])?;
        }

        Ok(())
    }

    fn fk_vulcheck() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "=".repeat(50));
        println!("Vulnerability Check:");
        println!("{}", "=".repeat(50));

        // 创建临时文件
        let vuln_log = PathBuf::from("vuln.log");
        let pass_tmp = PathBuf::from("pass.tmp");
        let mut vuln_file = File::create(&vuln_log)?;
        let mut pass_file = File::create(&pass_tmp)?;

        // 1. 检查 Redis 未授权访问
        println!("\n\x1b[31m1. Redis Unauthorized Access\x1b[0m\n");

        // 定义常见的 Redis 配置文件位置
        let common_paths = [
            "/etc/redis",
            "/usr/local/etc/redis",
            "/opt/redis",
            "/etc/redis.conf",
            "/usr/local/etc/redis.conf",
        ];

        for path in &common_paths {
            // 使用 find 命令，但限制深度和搜索范围
            let output = Command::new("find")
                .args([
                    path,
                    "-maxdepth",
                    "2", // 限制搜索深度
                    "-name",
                    "redis.conf",
                    "-type",
                    "f",
                ])
                .output()
                .map_err(|e| format!("Failed to execute find command: {e}"))?;

            if output.status.success() {
                for conf_path in String::from_utf8_lossy(&output.stdout).lines() {
                    if let Ok(file) = File::open(conf_path) {
                        let reader = BufReader::new(file);
                        for line in reader.lines().map_while(Result::ok) {
                            if line.contains("# requirepass") {
                                let output = format!("{conf_path}: {line}\n");
                                print!("{output}");
                                vuln_file.write_all(output.as_bytes())?;
                            }
                        }
                    }
                }
            }
        }

        // 2. 检查 Redis 弱密码
        println!("\n\x1b[31m2. Redis Weak Password Check\x1b[0m\n");
        for path in &common_paths {
            let output = Command::new("find")
                .args([path, "-maxdepth", "2", "-name", "redis.conf", "-type", "f"])
                .output()?;

            if output.status.success() {
                for conf_path in String::from_utf8_lossy(&output.stdout).lines() {
                    if let Ok(file) = File::open(conf_path) {
                        let reader = BufReader::new(file);
                        for line in reader.lines().map_while(Result::ok) {
                            if line.starts_with("requirepass") {
                                let parts: Vec<&str> = line.split_whitespace().collect();
                                if parts.len() >= 2 {
                                    println!("{conf_path}: requirepass ****");
                                    writeln!(pass_file, "{}", parts[1])?;
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("-------");

        // 检查常见弱密码
        let weak_passwords = [
            "admin123", "test", "123456", "admin", "root", "12345678", "111111", "p@ssw0rd",
            "test", "qwerty", "zxcvbnm", "123123", "12344321", "123qwe", "password", "1qaz",
            "000000", "666666", "888888",
        ];

        if let Ok(content) = fs::read_to_string(&pass_tmp) {
            for line in content.lines() {
                if weak_passwords.contains(&line) {
                    println!("[+] {line}");
                }
            }
        }

        // 清理临时文件
        if vuln_log.exists() {
            fs::remove_file(vuln_log)?;
        }
        if pass_tmp.exists() {
            fs::remove_file(pass_tmp)?;
        }

        println!();
        Ok(())
    }

    fn fk_rookit_analysis() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "=".repeat(50));
        println!("Rootkit Detection:");
        println!("{}", "=".repeat(50));

        // 创建输出目录
        fs::create_dir_all("./output")?;

        // 定义输出文件路径
        let chkrootkit_results = PathBuf::from("./output/chkrootkit_results.txt");
        let rkhunter_results = PathBuf::from("./output/rkhunter_results.txt");

        // 检查工具是否安装
        println!("\nChecking required tools...");
        let tools = ["chkrootkit", "rkhunter"];
        for tool in &tools {
            match Command::new("which").arg(tool).output() {
                Ok(output) if output.status.success() => {
                    println!("{tool} is installed");
                }
                _ => {
                    println!("\x1b[31mWarning: {tool} is not installed\x1b[0m");
                    println!("Please install it using your package manager");
                    continue;
                }
            }
        }

        // 运行 chkrootkit
        println!("\nRunning chkrootkit...");
        if let Ok(output) = Command::new("sudo").args(["chkrootkit"]).output() {
            fs::write(&chkrootkit_results, &output.stdout)?;
            println!("chkrootkit scan results saved to {chkrootkit_results:?}");

            // 显示重要发现
            println!("\nImportant findings from chkrootkit:");
            let content = String::from_utf8_lossy(&output.stdout);
            for line in content.lines() {
                if line.contains("INFECTED") || line.contains("WARNING") {
                    println!("\x1b[31m{line}\x1b[0m");
                }
            }
        } else {
            println!("Failed to run chkrootkit");
        }

        // 运行 rkhunter
        println!("\nRunning rkhunter...");
        if let Ok(output) = Command::new("sudo")
            .args(["rkhunter", "--check", "--sk"])
            .output()
        {
            fs::write(&rkhunter_results, &output.stdout)?;
            println!("rkhunter scan results saved to {rkhunter_results:?}");

            // 显示重要发现
            println!("\nImportant findings from rkhunter:");
            let content = String::from_utf8_lossy(&output.stdout);
            for line in content.lines() {
                if line.contains("Warning:") || line.contains('[') && line.contains("failed") {
                    println!("\x1b[31m{line}\x1b[0m");
                }
            }
        } else {
            println!("Failed to run rkhunter");
        }

        println!("\nDetailed results have been saved to the output directory");
        println!("Please review the full reports for more information");
        println!();
        Ok(())
    }

    fn fk_wsfinder(paths: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "=".repeat(50));
        println!("WebShell Detection:");
        println!("{}", "=".repeat(50));

        // 创建输出目录
        fs::create_dir_all("output")?;
        let output_file = PathBuf::from("output/webshell.txt");
        let mut file = File::create(&output_file)?;

        // WebShell 检测规则
        #[allow(clippy::needless_raw_string_hashes)]
        let php_rules = [
            r#"array_map\(|pcntl_exec\(|proc_open\(|popen\(|assert\(|phpspy|c99sh|milw0rm|eval\?\(|\(gunerpress|\(base64_decoolcode|spider_bc|shell_exec\(|passthru\(|base64_decode\s?\(|gzuncompress\s?\(|gzinflate|\(\$\$\w+|call_user_func\(|call_user_func_array\(|preg_replace_callback\(|preg_replace\(|register_shutdown_function\(|register_tick_function\(|mb_ereg_replace_callback\(|filter_var\(|ob_start\(|usort\(|uksort\(|uasort\(|GzinFlate\s?\(|\$\w+\(\d+\)\.\$\w+\(\d+\)\.|\$\w+=str_replace\(|eval/\*.*\*/\("#,
            r#"^(\xff\xd8|\x89\x50|GIF89a|GIF87a|BM|\x00\x00\x01\x00\x01)[\s\S]*<\?\s*php"#,
            r#"\b(assert|eval|system|exec|shell_exec|passthru|popen|proc_open|pcntl_exec)\b[/*\s]*\(+[/*\s]*((\$_(GET|POST|REQUEST|COOKIE)\[.{0,25})|(base64_decode|gzinflate|gzuncompress|gzdecode|str_rot13)[\s\(]*(\$_(GET|POST|REQUEST|COOKIE)\[.{0,25}))"#,
            r#"\$\s*(\w+)\s*=[\s\(\{]*(\$_(GET|POST|REQUEST|COOKIE)\[.{0,25});[\s\S]{0,200}\b(assert|eval|system|exec|shell_exec|passthru|popen|proc_open|pcntl_exec)\b[/*\s]*\(+[\s"/*]*(\$\s*\1|((base64_decode|gzinflate|gzuncompress|gzdecode|str_rot13)[\s\("]*\$\s*\1))"#,
            r#"\b(filter_var|filter_var_array)\b\s*\(.*FILTER_CALLBACK[^;]*((\$_(GET|POST|REQUEST|COOKIE|SERVER)\[.{0,25})|(eval|assert|ass\x65rt|system|exec|shell_exec|passthru|popen|proc_open|pcntl_exec))"#,
            r#"\b(assert|eval|system|exec|shell_exec|passthru|popen|proc_open|pcntl_exec|include)\b\s*\(\s*(file_get_contents\s*\(\s*)?['""]php://input"#,
        ];

        #[allow(clippy::needless_raw_string_hashes)]
        #[allow(clippy::items_after_statements)]
        const JSP_RULE: &str = r#"<%@\spage\simport=[\s\S]*\u00\d+\u00\d+|<%@\spage\simport=[\s\S]*Runtime.getRuntime\(\).exec\(request.getParameter\(|Runtime.getRuntime\(\)"#;

        for path in paths {
            println!("\nChecking directory: {path}");
            if !PathBuf::from(path).is_dir() {
                println!("Directory not found: {path}");
                continue;
            }

            // 检查 PHP WebShell
            println!("\n\x1b[31m1. PHP WebShells\x1b[0m");
            for rule in &php_rules {
                let output = Command::new("find")
                    .args([
                        path, "-type", "f", "-name", "*.php", "-exec", "grep", "-P", "-i", rule,
                        "{}", "+",
                    ])
                    .output()?;

                if !output.stdout.is_empty() {
                    let result = String::from_utf8_lossy(&output.stdout);
                    println!("{result}");
                    writeln!(file, "{result}")?;
                }
            }

            // 检查 JSP WebShell
            println!("\n\x1b[31m2. JSP WebShells\x1b[0m");
            let output = Command::new("find")
                .args([
                    path, "-type", "f", "-name", "*.jsp", "-exec", "grep", "-P", "-i", JSP_RULE,
                    "{}", "+",
                ])
                .output()?;

            if !output.stdout.is_empty() {
                let result = String::from_utf8_lossy(&output.stdout);
                println!("{result}");
                writeln!(file, "{result}")?;
            }
        }

        println!("\nDetailed results have been saved to output/webshell.txt");
        println!();
        Ok(())
    }
}
