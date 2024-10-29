use clap::Parser;
use regex::Regex;
use reqwest::Client;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Parser, Debug, Clone)]
#[command(name="misc", author, version, about="", long_about = None)]
pub struct Misc {
    #[arg(
        short,
        long,
        help = "check page live status (URL or file path)",
        required = false
    )]
    pub code: Option<FileOrUrl>,

    #[arg(short, long, help = "check user information", required = false)]
    pub sqletlog: Option<PathBuf>,

    #[arg(short, long, help = "set crontab information", required = false)]
    pub auto_run: Option<String>,

    #[arg(
        short,
        long,
        help = "custom command define test",
        default_value = "[default:~/.whok/chief-inspector.conf]",
        required = false
    )]
    pub ext: PathBuf,
}

#[derive(Debug, Clone)]
pub enum FileOrUrl {
    File(PathBuf),
    Url(url::Url),
}

impl std::str::FromStr for FileOrUrl {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(url) = url::Url::parse(s) {
            Ok(FileOrUrl::Url(url))
        } else {
            Ok(FileOrUrl::File(PathBuf::from(s)))
        }
    }
}

impl Misc {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref code) = self.code {
            match code {
                FileOrUrl::File(path) => Self::fk_http_scan_file(path).await?,
                FileOrUrl::Url(url) => Self::fk_http_scan_url(url).await?,
            }
        }

        if let Some(ref sqletlog) = self.sqletlog {
            Self::fk_weblog_sqlianalysis(sqletlog)?;
        }

        // if let Some(ref auto_run) = self.auto_run {
        //     // 处理 auto_run
        // }

        Ok(())
    }

    async fn fk_http_scan_url(url: &url::Url) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "=".repeat(50));
        println!("Web Page Status Check:");
        println!("{}", "=".repeat(50));

        // 创建输出目录
        fs::create_dir_all("output")?;
        let mut output_file = File::create("output/http_info.txt")?;

        // 创建 HTTP 客户端
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36")
            .timeout(Duration::from_secs(10))
            .danger_accept_invalid_certs(true)
            .build()?;

        // 发送请求
        let response = client.get(url.as_str()).send().await?;
        let status = response.status();
        let body = response.text().await?;
        let bytes = body.len();

        // 提取标题
        let title = extract_title(&body).unwrap_or_else(|| "No title found".to_string());

        // 输出结果
        let output = format!(
            "[INFO] {} [{}] [{}] [{}]\n",
            url.as_str(),
            status.as_u16(),
            bytes,
            title
        );
        println!("\x1b[35m{output}\x1b[0m");
        output_file.write_all(output.as_bytes())?;

        Ok(())
    }

    async fn fk_http_scan_file(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "=".repeat(50));
        println!("Web Pages Status Check:");
        println!("{}", "=".repeat(50));

        // 创建输出目录
        fs::create_dir_all("output")?;
        let mut output_file = File::create("output/http_info.txt")?;

        // 创建 HTTP 客户端
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36")
            .timeout(Duration::from_secs(10))
            .danger_accept_invalid_certs(true)
            .build()?;

        // 读取URL列表
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let url = line?;
            if let Ok(url) = url::Url::parse(&url) {
                match client.get(url.as_str()).send().await {
                    Ok(response) => {
                        let status = response.status();
                        let body = response.text().await?;
                        let bytes = body.len();
                        let title =
                            extract_title(&body).unwrap_or_else(|| "No title found".to_string());

                        let output = format!(
                            "[INFO] {} [{}] [{}] [{}]\n",
                            url.as_str(),
                            status.as_u16(),
                            bytes,
                            title
                        );
                        println!("\x1b[35m{output}\x1b[0m");
                        output_file.write_all(output.as_bytes())?;
                    }
                    Err(e) => {
                        let output = format!("[ERROR] {} - {}\n", url.as_str(), e);
                        println!("\x1b[31m{output}\x1b[0m");
                        output_file.write_all(output.as_bytes())?;
                    }
                }
            }
        }

        Ok(())
    }

    fn fk_weblog_sqlianalysis(log_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "=".repeat(50));
        println!("SQL Injection Log Analysis:");
        println!("{}", "=".repeat(50));

        // SQL 注入特征
        let sql_patterns = [
            "select",
            "union",
            "and",
            "or",
            "from",
            "where",
            "order by",
            "group by",
            "having",
            "limit",
            "offset",
            "delete",
            "drop",
            "update",
            "insert",
            "'",
            "\"",
            "/*",
            "*/",
            "--",
            "#",
            "\\",
            "=",
            "%27",
            "%22",
            "information_schema",
            "sysobjects",
            "syscolumns",
            "/*!",
            "declare",
            "exec",
            "xp_cmdshell",
            "sp_",
            "waitfor",
            "delay",
            "benchmark",
            "sleep",
            "load_file",
            "outfile",
            "dumpfile",
        ];

        // 检查默认日志路径
        let default_paths = [
            "/var/log/apache2/access.log",
            "/var/log/apache/access.log",
            "/var/log/nginx/access.log",
        ];

        if log_path.as_os_str().is_empty() {
            // 检查默认路径
            for path in &default_paths {
                let path = PathBuf::from(path);
                if path.exists() {
                    println!("\nChecking {}", path.display());
                    Self::analyze_log_file(&path, &sql_patterns)?;
                } else {
                    println!("\nFile not found: {}", path.display());
                }
            }
        } else {
            // 检查指定的日志文件
            println!("\nChecking {}", log_path.display());
            if log_path.exists() {
                Self::analyze_log_file(log_path, &sql_patterns)?;
            } else {
                println!("File not found: {}", log_path.display());
            }
        }

        Ok(())
    }

    fn analyze_log_file(
        path: &PathBuf,
        patterns: &[&str],
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 创建输出目录
        fs::create_dir_all("output")?;
        let mut output_file = File::create("output/sqli_analysis.txt")?;

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut suspicious_count = 0;

        println!("\nAnalyzing log file for SQL injection attempts...");
        println!("{}", "-".repeat(50));

        for line in reader.lines() {
            let line = line?;
            let line_lower = line.to_lowercase();

            // 检查是否包含 SQL 注入特征
            let mut found_patterns = Vec::new();
            for pattern in patterns {
                if line_lower.contains(&pattern.to_lowercase()) {
                    found_patterns.push(*pattern);
                }
            }

            // 如果找到可疑模式
            if !found_patterns.is_empty() {
                suspicious_count += 1;
                let output = format!(
                    "[SUSPICIOUS] Found patterns: {}\nLog entry: {}\n{}\n",
                    found_patterns.join(", "),
                    line,
                    "-".repeat(80)
                );
                println!("\x1b[31m{}\x1b[0m", output);
                output_file.write_all(output.as_bytes())?;
            }
        }

        println!("\nAnalysis Summary:");
        println!("Total suspicious entries found: {suspicious_count}");
        println!("Detailed results have been saved to output/sqli_analysis.txt");

        Ok(())
    }
}

fn extract_title(html: &str) -> Option<String> {
    let re = Regex::new(r"<title>(.*?)</title>").ok()?;
    re.captures(html).map(|cap| cap[1].to_string())
}
