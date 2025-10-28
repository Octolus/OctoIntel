use clap::Parser;
use colored::Colorize;
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use ipnetwork::Ipv4Network;
use regex::Regex;
use std::fs;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use sysinfo::System;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

#[derive(Parser, Debug)]
#[command(
    name = "octointel",
    author = "Octolus from OctoVPN team",
    version = "2.0.0",
    about = "Production-ready reverse proxy backend IP scanner",
    long_about = "Ultra-fast, production-ready IP scanner for discovering backend servers behind CDNs and reverse proxies. \
                  Supports multiple HTTP methods, content matching, and dynamic CPU scaling."
)]
struct Args {
    /// Target domain to scan for
    #[arg(value_name = "DOMAIN")]
    domain: String,

    /// IP ranges to scan (CIDR notation, e.g., 35.207.0.0/16)
    #[arg(short, long, value_delimiter = ',')]
    ranges: Option<Vec<String>>,

    /// Path to file containing IP ranges (one CIDR per line)
    #[arg(short = 'f', long, value_name = "FILE")]
    ip_file: Option<PathBuf>,

    /// HTTP method to use: HEAD, GET, or POST
    #[arg(short = 'm', long, default_value = "HEAD", value_parser = ["HEAD", "GET", "POST"])]
    method: String,

    /// HTTP status code to match (default: 202)
    #[arg(long, default_value = "202")]
    status_code: u16,

    /// Search for specific content in HTML response (regex supported)
    #[arg(short = 'c', long)]
    content_match: Option<String>,

    /// POST request body (when using POST method)
    #[arg(long)]
    post_body: Option<String>,

    /// Custom HTTP headers (format: "Header: Value", can be specified multiple times)
    #[arg(long = "header", value_name = "HEADER")]
    headers: Option<Vec<String>>,

    /// Connection timeout in milliseconds (auto-detected if not specified)
    #[arg(short, long)]
    timeout: Option<u64>,

    /// Maximum concurrent connections (auto-detected if not specified)
    #[arg(short, long)]
    workers: Option<usize>,

    /// Scan a single IP address
    #[arg(long)]
    single_ip: Option<String>,

    /// Stop immediately after finding first match
    #[arg(long, default_value = "true")]
    stop_on_find: bool,

    /// HTTP port to scan (default: 80)
    #[arg(short = 'p', long, default_value = "80")]
    port: u16,

    /// Use HTTPS instead of HTTP
    #[arg(long)]
    https: bool,

    /// Verbose output for debugging
    #[arg(short, long)]
    verbose: bool,
}

/// Scanner configuration and state management
///
/// Holds all configuration needed for scanning IP ranges, including:
/// - Target domain and connection parameters
/// - HTTP request configuration (method, headers, body)
/// - Content matching rules (regex patterns)
/// - Concurrency and performance settings
struct Scanner {
    domain: Arc<String>,
    timeout: Duration,
    workers: usize,
    stop_flag: Arc<AtomicBool>,
    found_count: Arc<AtomicU64>,
    request_bytes: Arc<Vec<u8>>,
    method: Arc<String>,
    status_code: u16,
    content_regex: Arc<Option<Regex>>,
    port: u16,
    verbose: bool,
}

impl Scanner {
    /// Create a new Scanner instance with all configuration
    ///
    /// # Arguments
    /// * `domain` - Target domain to scan for
    /// * `timeout` - Connection timeout duration
    /// * `workers` - Number of concurrent workers
    /// * `method` - HTTP method (HEAD, GET, or POST)
    /// * `status_code` - Expected HTTP status code to match
    /// * `content_match` - Optional regex pattern to search in response
    /// * `headers` - Optional custom HTTP headers
    /// * `post_body` - Optional POST request body
    /// * `port` - Target port number
    /// * `verbose` - Enable verbose debug output
    ///
    /// # Returns
    /// * `Ok(Scanner)` - Configured scanner ready to use
    /// * `Err` - If configuration is invalid (bad regex, invalid method, etc.)
    fn new(
        domain: String,
        timeout: Duration,
        workers: usize,
        method: String,
        status_code: u16,
        content_match: Option<String>,
        headers: Option<Vec<String>>,
        post_body: Option<String>,
        port: u16,
        verbose: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Build HTTP request with specified method
        let mut request = match method.as_str() {
            "HEAD" => format!("HEAD / HTTP/1.1\r\nHost: {}\r\n", domain),
            "GET" => format!("GET / HTTP/1.1\r\nHost: {}\r\n", domain),
            "POST" => {
                let body = post_body.as_deref().unwrap_or("");
                format!(
                    "POST / HTTP/1.1\r\nHost: {}\r\nContent-Length: {}\r\n",
                    domain,
                    body.len()
                )
            }
            _ => return Err(format!("Unsupported HTTP method: {}", method).into()),
        };

        // Add custom headers if provided
        if let Some(ref custom_headers) = headers {
            for header in custom_headers {
                if !header.contains(':') {
                    return Err(format!(
                        "Invalid header format: '{}'. Expected 'Header: Value'",
                        header
                    )
                    .into());
                }
                request.push_str(header);
                request.push_str("\r\n");
            }
        }

        // Add standard headers
        request.push_str("Connection: close\r\n");
        request.push_str("User-Agent: octointel/2.0\r\n");

        // Complete headers and add body for POST
        request.push_str("\r\n");
        if method == "POST" {
            if let Some(body) = post_body {
                request.push_str(&body);
            }
        }

        // Compile regex if content matching is enabled
        let content_regex = if let Some(pattern) = content_match {
            match Regex::new(&pattern) {
                Ok(re) => Some(re),
                Err(e) => return Err(format!("Invalid regex pattern: {}", e).into()),
            }
        } else {
            None
        };

        Ok(Self {
            domain: Arc::new(domain),
            timeout,
            workers,
            stop_flag: Arc::new(AtomicBool::new(false)),
            found_count: Arc::new(AtomicU64::new(0)),
            request_bytes: Arc::new(request.into_bytes()),
            method: Arc::new(method),
            status_code,
            content_regex: Arc::new(content_regex),
            port,
            verbose,
        })
    }

    /// Scan a single IP address for the target domain
    ///
    /// # Arguments
    /// * `ip` - IPv4 address to scan
    ///
    /// # Returns
    /// * `Some((ip, info))` - If match found, returns IP and match details
    /// * `None` - If no match or connection failed
    ///
    /// # Behavior
    /// - Connects to ip:port via TCP
    /// - Sends configured HTTP request (HEAD/GET/POST)
    /// - Checks for matching status code
    /// - Optionally validates content with regex
    /// - Returns immediately if stop_flag is set
    async fn scan_ip(&self, ip: Ipv4Addr) -> Option<(String, String)> {
        // Check stop flag early (avoid unnecessary work)
        if self.stop_flag.load(Ordering::Relaxed) {
            return None;
        }

        let socket_addr = SocketAddr::new(IpAddr::V4(ip), self.port);

        if self.verbose {
            println!("{} Scanning {}:{}", "→".bright_cyan(), ip, self.port);
        }

        // Attempt connection with timeout
        match timeout(self.timeout, TcpStream::connect(socket_addr)).await {
            Ok(Ok(mut stream)) => {
                // Disable Nagle's algorithm for faster small packets
                let _ = stream.set_nodelay(true);

                // Send HTTP request
                if let Err(e) = stream.write_all(&self.request_bytes).await {
                    if self.verbose {
                        eprintln!("{} Failed to write to {}: {}", "✗".red(), ip, e);
                    }
                    return None;
                }

                // Read response - use larger buffer for content matching
                let buffer_size = if self.content_regex.is_some() || self.method.as_str() == "GET" {
                    8192 // 8KB for full response content
                } else {
                    512 // Small buffer for status line only
                };

                let mut buffer = vec![0u8; buffer_size];

                match timeout(self.timeout, stream.read(&mut buffer)).await {
                    Ok(Ok(bytes_read)) => {
                        if bytes_read == 0 {
                            return None;
                        }

                        // Convert to string for parsing
                        let response = String::from_utf8_lossy(&buffer[..bytes_read]);

                        // Parse HTTP status code
                        let status_match = format!(" {} ", self.status_code);
                        let has_status = response.contains(&status_match);

                        // Check content if regex is provided
                        let content_matched = if let Some(ref regex) = *self.content_regex {
                            regex.is_match(&response)
                        } else {
                            true // No content filter, so consider it matched
                        };

                        if has_status && content_matched {
                            let info = if self.content_regex.is_some() {
                                format!("Status: {}, Content matched", self.status_code)
                            } else {
                                format!("Status: {}", self.status_code)
                            };
                            return Some((ip.to_string(), info));
                        }

                        if self.verbose && has_status {
                            println!(
                                "{} {} returned {} but content didn't match",
                                "ℹ".bright_blue(),
                                ip,
                                self.status_code
                            );
                        }
                    }
                    Ok(Err(e)) => {
                        if self.verbose {
                            eprintln!("{} Failed to read from {}: {}", "✗".red(), ip, e);
                        }
                    }
                    Err(_) => {
                        if self.verbose {
                            eprintln!("{} Read timeout for {}", "✗".red(), ip);
                        }
                    }
                }
            }
            Ok(Err(e)) => {
                if self.verbose {
                    eprintln!("{} Connection failed for {}: {}", "✗".red(), ip, e);
                }
            }
            Err(_) => {
                if self.verbose {
                    eprintln!("{} Connection timeout for {}", "✗".red(), ip);
                }
            }
        }

        None
    }

    /// Scan an entire IP range (CIDR notation)
    ///
    /// # Arguments
    /// * `range` - CIDR notation (e.g., "35.207.0.0/16")
    /// * `stop_on_find` - Whether to stop after first match
    ///
    /// # Returns
    /// * Vector of (ip, info) tuples for all matches found
    ///
    /// # Behavior
    /// - Parses CIDR range into individual IPs
    /// - Creates concurrent scan tasks (up to `workers` parallel)
    /// - Shows progress bar with real-time stats
    /// - Stops early if `stop_on_find` is true and match is found
    async fn scan_range(&self, range: &str, stop_on_find: bool) -> Vec<(String, String)> {
        let network: Ipv4Network = match range.parse() {
            Ok(net) => net,
            Err(e) => {
                eprintln!("{} Failed to parse range {}: {}", "✗".red(), range, e);
                return Vec::new();
            }
        };

        let total_ips = network.size() as u64;
        let ips: Vec<Ipv4Addr> = network.iter().collect();

        println!(
            "\n{}\n{} Scanning {} IPs in range {}\n{}",
            "=".repeat(60).bright_cyan(),
            "➤".bright_green(),
            total_ips,
            range.bright_yellow(),
            "=".repeat(60).bright_cyan()
        );

        let progress = ProgressBar::new(total_ips);
        progress.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) | {per_sec} IPs/sec | ETA: {eta}")
                .unwrap()
                .progress_chars("█▓▒░"),
        );

        let found_ips = Arc::new(tokio::sync::Mutex::new(Vec::new()));
        let found_ips_clone = found_ips.clone();
        let stop_flag = self.stop_flag.clone();
        let found_count = self.found_count.clone();
        let request_bytes = self.request_bytes.clone();
        let domain = self.domain.clone();
        let method = self.method.clone();
        let status_code = self.status_code;
        let content_regex = self.content_regex.clone();
        let port = self.port;
        let verbose = self.verbose;

        // Create stream of scan tasks
        let mut stream = stream::iter(ips)
            .map(|ip| {
                let scanner = Scanner {
                    domain: domain.clone(),
                    timeout: self.timeout,
                    workers: self.workers,
                    stop_flag: stop_flag.clone(),
                    found_count: found_count.clone(),
                    request_bytes: request_bytes.clone(),
                    method: method.clone(),
                    status_code,
                    content_regex: content_regex.clone(),
                    port,
                    verbose,
                };
                let progress = progress.clone();
                let found_ips = found_ips_clone.clone();
                let stop_flag_inner = stop_flag.clone();
                let found_count_inner = found_count.clone();

                async move {
                    let result = scanner.scan_ip(ip).await;
                    progress.inc(1);

                    if let Some((found_ip, info)) = result {
                        println!(
                            "\n{} {} - {}",
                            "✓ FOUND:".bright_green().bold(),
                            found_ip.bright_yellow().bold(),
                            info.bright_white()
                        );

                        found_ips
                            .lock()
                            .await
                            .push((found_ip.clone(), info.clone()));
                        found_count_inner.fetch_add(1, Ordering::Relaxed);

                        if stop_on_find {
                            stop_flag_inner.store(true, Ordering::Relaxed);
                            println!(
                                "\n{} Backend IP found! Stopping scan immediately...\n",
                                "⚠".bright_yellow()
                            );
                        }

                        Some((found_ip, info))
                    } else {
                        None
                    }
                }
            })
            .buffer_unordered(self.workers);

        // Process results
        while let Some(_) = stream.next().await {
            if self.stop_flag.load(Ordering::Relaxed) {
                break;
            }
        }

        progress.finish_and_clear();

        let found = found_ips.lock().await;
        found.clone()
    }
}

/// Detect optimal system settings for maximum performance
///
/// # Returns
/// * (workers, timeout_ms, worker_threads) tuple
///
/// # Auto-Detection Logic
/// - CPU cores: Used for Tokio worker threads
/// - RAM: Determines max concurrent connections
///   - 16GB+: 10000 workers
///   - 8-16GB: 5000 workers
///   - 4-8GB: 2000 workers
///   - <4GB: 1000 workers
/// - Timeout: Adjusted based on system capabilities
///   - High-end: 300ms (aggressive)
///   - Mid-range: 500ms (balanced)
///   - Low-end: 1000ms (conservative)
fn detect_optimal_settings() -> (usize, u64, usize) {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Detect CPU cores
    let cpu_count = sys.cpus().len();

    // Detect available memory (in GB)
    let total_memory_gb = sys.total_memory() / (1024 * 1024 * 1024);

    // Calculate optimal worker threads (use all cores but cap at 16)
    let worker_threads = cpu_count.min(16).max(4);

    // Calculate optimal concurrent workers based on memory
    // Each connection uses ~50KB, so we can estimate max safe connections
    let workers = if total_memory_gb >= 16 {
        // High-end: 16GB+ RAM
        10000
    } else if total_memory_gb >= 8 {
        // Mid-range: 8-16GB RAM
        5000
    } else if total_memory_gb >= 4 {
        // Low-end: 4-8GB RAM
        2000
    } else {
        // Very low: <4GB RAM
        1000
    };

    // Calculate optimal timeout based on expected performance
    let timeout = if total_memory_gb >= 16 && cpu_count >= 8 {
        // High-end system: aggressive timeout
        300
    } else if total_memory_gb >= 8 && cpu_count >= 4 {
        // Mid-range: balanced
        500
    } else {
        // Low-end: conservative
        1000
    };

    println!("{} Auto-detected system capabilities:", "ℹ".bright_blue());
    println!("  {} CPU Cores: {}", "→".bright_cyan(), cpu_count);
    println!("  {} RAM: {} GB", "→".bright_cyan(), total_memory_gb);
    println!(
        "  {} Tokio Worker Threads: {}",
        "→".bright_cyan(),
        worker_threads
    );
    println!("  {} Concurrent Workers: {}", "→".bright_cyan(), workers);
    println!("  {} Connection Timeout: {}ms", "→".bright_cyan(), timeout);
    println!();

    (workers, timeout, worker_threads)
}

/// Load IP ranges from a text file (one CIDR notation per line)
///
/// # Arguments
/// * `file_path` - Path to the file containing IP ranges
///
/// # Returns
/// * `Ok(Vec<String>)` - Vector of valid CIDR ranges
/// * `Err` - If file cannot be read or contains no valid ranges
///
/// # Format
/// - One CIDR range per line (e.g., "35.207.0.0/16")
/// - Lines starting with '#' or '//' are treated as comments
/// - Empty lines are ignored
fn load_ip_ranges_from_file(
    file_path: &PathBuf,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!(
        "{} Loading IP ranges from: {}",
        "ℹ".bright_blue(),
        file_path.display()
    );

    let content = fs::read_to_string(file_path)?;
    let mut ranges = Vec::new();
    let mut line_num = 0;

    for line in content.lines() {
        line_num += 1;
        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }

        // Validate CIDR notation before adding to list
        match trimmed.parse::<Ipv4Network>() {
            Ok(_) => ranges.push(trimmed.to_string()),
            Err(e) => {
                eprintln!(
                    "{} Invalid CIDR format on line {}: '{}' - {}",
                    "⚠".bright_yellow(),
                    line_num,
                    trimmed,
                    e
                );
            }
        }
    }

    if ranges.is_empty() {
        return Err(format!("No valid IP ranges found in {}", file_path.display()).into());
    }

    println!(
        "{} Loaded {} valid IP range(s) from file",
        "✓".bright_green(),
        ranges.len()
    );

    Ok(ranges)
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let args = Args::parse();

    // Print banner
    println!(
        "\n{}\n{} {} v{}\n{} Ultra-fast reverse proxy backend scanner\n{}",
        "=".repeat(60).bright_cyan(),
        "🔍".to_string(),
        "OctoIntel".bright_yellow().bold(),
        "2.0.0",
        "⚡".to_string(),
        "=".repeat(60).bright_cyan()
    );

    // Validate arguments
    if args.method == "POST" && args.post_body.is_none() && args.verbose {
        println!("{} Using POST method without a body", "⚠".bright_yellow());
    }

    // Auto-detect optimal settings if not provided
    let (optimal_workers, optimal_timeout, _worker_threads) = detect_optimal_settings();

    // Use provided values or auto-detected ones
    let workers = args.workers.unwrap_or(optimal_workers);
    let timeout = args.timeout.unwrap_or(optimal_timeout);

    // Create scanner with all the new options
    let scanner = match Scanner::new(
        args.domain.clone(),
        Duration::from_millis(timeout),
        workers,
        args.method.clone(),
        args.status_code,
        args.content_match.clone(),
        args.headers.clone(),
        args.post_body.clone(),
        args.port,
        args.verbose,
    ) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} Failed to create scanner: {}", "✗".red(), e);
            std::process::exit(1);
        }
    };

    // Handle single IP scan
    if let Some(single_ip) = args.single_ip {
        println!(
            "{} Scanning single IP: {}:{}",
            "➤".bright_green(),
            single_ip,
            args.port
        );

        match single_ip.parse::<Ipv4Addr>() {
            Ok(ip) => {
                if let Some((found, info)) = scanner.scan_ip(ip).await {
                    println!("{} {} - {}", "✓".green(), found, info);
                } else {
                    println!("{} No matching response from {}", "✗".red(), single_ip);
                }
            }
            Err(e) => {
                eprintln!("{} Invalid IP address: {}", "✗".red(), e);
                std::process::exit(1);
            }
        }
        return;
    }

    // Get IP ranges to scan - priority: file > cli args > error
    let ip_ranges = if let Some(file_path) = args.ip_file {
        // Load from file
        match load_ip_ranges_from_file(&file_path) {
            Ok(ranges) => ranges,
            Err(e) => {
                eprintln!("{} Failed to load IP ranges from file: {}", "✗".red(), e);
                std::process::exit(1);
            }
        }
    } else if let Some(ranges) = args.ranges {
        // Use CLI-provided ranges
        ranges
    } else {
        // No IP ranges specified - require user input
        eprintln!("{} Error: No IP ranges specified!", "✗".red());
        eprintln!();
        eprintln!("Please provide IP ranges using one of these methods:");
        eprintln!("  1. File:       --ip-file ips.txt");
        eprintln!("  2. CLI args:   --ranges 35.207.0.0/16,10.0.0.0/24");
        eprintln!("  3. Single IP:  --single-ip 35.207.76.249");
        eprintln!();
        eprintln!("Example: octointel example.com --ip-file ips.txt");
        eprintln!("See ips.txt.example for sample IP ranges");
        std::process::exit(1);
    };

    // Print scan configuration
    println!(
        "\n{}\n{} Scan Configuration:\n{}",
        "=".repeat(60).bright_cyan(),
        "⚙".to_string(),
        "=".repeat(60).bright_cyan()
    );
    println!(
        "  {} Target domain: {}",
        "→".bright_cyan(),
        args.domain.bright_yellow()
    );
    println!(
        "  {} HTTP method: {}",
        "→".bright_cyan(),
        args.method.bright_yellow()
    );
    println!(
        "  {} Port: {}",
        "→".bright_cyan(),
        args.port.to_string().bright_yellow()
    );
    println!(
        "  {} Target status: {}",
        "→".bright_cyan(),
        args.status_code.to_string().bright_yellow()
    );

    if let Some(ref content) = args.content_match {
        println!(
            "  {} Content match: {}",
            "→".bright_cyan(),
            content.bright_yellow()
        );
    }

    if let Some(ref headers) = args.headers {
        println!(
            "  {} Custom headers: {} header(s)",
            "→".bright_cyan(),
            headers.len()
        );
    }

    println!("  {} IP ranges: {}", "→".bright_cyan(), ip_ranges.len());
    println!("  {} Concurrent workers: {}", "→".bright_cyan(), workers);
    println!("  {} Timeout: {}ms", "→".bright_cyan(), timeout);

    let start_time = Instant::now();
    let mut all_found_ips = Vec::new();

    // Scan each range
    for range in &ip_ranges {
        let found = scanner.scan_range(range, args.stop_on_find).await;
        all_found_ips.extend(found.clone());

        // Stop if we found IPs and stop_on_find is enabled
        if args.stop_on_find && !found.is_empty() {
            println!(
                "\n{} Found backend IP(s) - stopping all remaining scans\n",
                "⚠".bright_yellow()
            );
            break;
        }
    }

    let elapsed = start_time.elapsed();

    println!(
        "\n{}\n{} Scan completed in {:.2}s\n{}",
        "=".repeat(60).bright_cyan(),
        "✓".bright_green(),
        elapsed.as_secs_f64(),
        "=".repeat(60).bright_cyan()
    );

    if all_found_ips.is_empty() {
        println!("{} No matching IPs found", "✗".red());
    } else {
        println!(
            "{} Found {} backend IP(s):",
            "✓".bright_green(),
            all_found_ips.len()
        );
        for (ip, info) in all_found_ips {
            println!("  {} {} - {}", "→".bright_cyan(), ip.bright_yellow(), info);
        }
    }
}
