# 🔍 OctoIntel

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-blue)](https://github.com/Octolus/OctoIntel)

**Production-ready**, ultra-fast reverse proxy backend IP scanner for discovering backend servers behind CDNs and reverse proxies like Cloudflare.

*By Octolus from OctoVPN team*

## ⚡ Features

- 🚀 **Blazing Fast** - 2000-10000+ concurrent connections with dynamic CPU scaling
- 📁 **File Input** - Load IP ranges from text files (CIDR notation)
- 🌐 **Multiple HTTP Methods** - Support for HEAD, GET, and POST requests
- 🔍 **Content Matching** - Search for specific patterns in HTML responses (regex supported)
- 🎯 **Flexible Status Codes** - Match any HTTP status code (default: 202)
- 📊 **Progress Tracking** - Real-time progress bars with speed and ETA
- 🎨 **Beautiful Output** - Colored terminal UI with detailed results
- ⚙️ **Auto-Optimization** - Automatically detects CPU cores and RAM for optimal performance
- 🔧 **Custom Headers** - Add custom HTTP headers to requests
- 🌍 **Cross-Platform** - Works on Windows, Linux, and macOS

## 📋 Requirements

- **Rust 1.70+** (for building from source)
- **Windows 10/11**, **Linux**, or **macOS**
- At least **4GB RAM** (8GB+ recommended for large scans)

## 🔧 Installation

### Build from Source

```bash
# Clone the repository
git clone https://github.com/Octolus/OctoIntel.git
cd OctoIntel

# Build release binary
cargo build --release

# Binary will be at: target/release/octointel.exe (Windows)
#                 or: target/release/octointel (Linux/macOS)
```

## 🎯 Real-World Use Case: Finding Backend Behind Cloudflare

When a website is behind Cloudflare (or similar CDN), you know the domain but need to find the actual backend server IP.

### Scenario Setup
```
Your Domain: example.com
Current Status: Behind Cloudflare (proxy enabled)
Goal: Find the real backend IP address
```

### Method 1: Fast Discovery with HEAD Request

Use HEAD when you know the backend returns a specific status code (like 202 Accepted):

```bash
# Quick scan looking for 202 status
octointel example.com \
  --ranges 35.207.0.0/16 \
  --method HEAD \
  --status-code 202
```

**When to use HEAD:**
- ✅ Fast scanning (doesn't download content)
- ✅ Backend returns unique status code
- ✅ You don't need to check page content
- ❌ Limited to HTTP status line only

### Method 2: Content-Based Discovery with GET Request

Use GET when you need to verify content matches your site:

```bash
# Find backend by checking for your site's title
octointel example.com \
  --method GET \
  --content-match "<title>My Site Title</title>" \
  --ranges 35.207.0.0/16
```

**When to use GET:**
- ✅ Need to verify page content
- ✅ Can check for unique identifiers
- ✅ More accurate (reduces false positives)
- ❌ Slower (downloads full response)

### Method 3: Regex Pattern Matching

Search for flexible patterns in the HTML:

```bash
# Match title with regex (case-insensitive, flexible)
octointel example.com \
  --method GET \
  --content-match "<title>.*example.*</title>" \
  --ranges 35.207.0.0/16
```

Common patterns to search for:
```bash
# Look for specific server headers
--content-match "Server: nginx/1.18.0"

# Find WordPress sites
--content-match "wp-content|wp-includes"

# Search for unique app identifiers
--content-match "data-app-id=\"abc123\""

# Match copyright text
--content-match "Copyright.*YourCompany"
```

## 📝 IP Ranges File Format

Create `ips.txt` with CIDR ranges to scan:

```text
# ips.txt - IP ranges to scan
# Lines starting with # are comments

# Google Cloud Platform - Common regions
35.207.0.0/16
35.208.0.0/16
35.209.0.0/16

# AWS - US East
3.208.0.0/12
52.0.0.0/14

# Digital Ocean
159.65.0.0/16
167.99.0.0/16

# Cloudflare (if backend is also on CF)
104.16.0.0/12
172.64.0.0/13
```

Then scan using the file:

```bash
octointel example.com --ip-file ips.txt
```

See `ips.txt.example` for comprehensive cloud provider ranges.

## 🚀 Quick Start Examples

### Example 1: Basic Scan with Default Ranges

```bash
# Uses built-in Google Cloud ranges
octointel example.com
```

### Example 2: Custom IP Ranges (Cloudflare Scenario)

```bash
# You know your backend is on Google Cloud US-West
octointel example.com \
  --ranges 35.207.0.0/16,35.208.0.0/16,35.209.0.0/16
```

### Example 3: Find Backend by Page Title

```bash
# Your site has unique title, use GET to verify
octointel example.com \
  --method GET \
  --content-match "<title>Welcome to Example Corp</title>" \
  --ip-file ips.txt
```

### Example 4: Check for Specific HTTP Header

```bash
# Backend returns custom header
octointel example.com \
  --method HEAD \
  --content-match "X-Custom-Backend: production" \
  --ranges 10.0.0.0/16
```

### Example 5: Search for Server Signature

```bash
# Look for specific nginx version
octointel example.com \
  --method GET \
  --content-match "Server:.*nginx/1\\.18" \
  --ranges 35.207.0.0/20
```

### Example 6: High-Speed Scan (Known Status Code)

```bash
# Backend always returns 202 for your domain
octointel example.com \
  --method HEAD \
  --status-code 202 \
  --workers 10000 \
  --timeout 300 \
  --ip-file google-cloud.txt
```

## 📖 HEAD vs GET: When to Use Which?

### Use HEAD When:
```bash
octointel example.com --method HEAD --status-code 202
```

✅ **Advantages:**
- Extremely fast (no content download)
- Low bandwidth usage
- Efficient for large IP ranges
- Good when backend has unique status code

❌ **Limitations:**
- Only checks HTTP status line and headers
- Cannot verify page content
- More false positives

### Use GET When:
```bash
octointel example.com --method GET --content-match "<title>Your Site</title>"
```

✅ **Advantages:**
- Can verify actual page content
- More accurate results
- Can use regex for flexible matching
- Reduces false positives

❌ **Limitations:**
- Slower (downloads full response)
- Higher bandwidth usage
- May need larger buffer for big pages

## 🎛️ Command-Line Options

### Basic Options

| Option | Description | Example |
|--------|-------------|---------|
| `DOMAIN` | Target domain (required) | `example.com` |
| `-r, --ranges` | IP ranges to scan (CIDR) | `-r 35.207.0.0/16,35.208.0.0/16` |
| `-f, --ip-file` | Load ranges from file | `-f ips.txt` |
| `-m, --method` | HTTP method (HEAD/GET/POST) | `-m GET` |
| `--status-code` | Status code to match | `--status-code 200` |
| `-c, --content-match` | Search pattern (regex) | `-c "<title>.*</title>"` |
| `-p, --port` | Target port | `-p 8080` |
| `-v, --verbose` | Debug output | `-v` |

### Performance Options

| Option | Description | Default |
|--------|-------------|---------|
| `-w, --workers` | Concurrent connections | Auto (2000-10000) |
| `-t, --timeout` | Timeout in milliseconds | Auto (300-1000) |
| `--stop-on-find` | Stop after first match | `true` |

### Advanced Options

| Option | Description | Example |
|--------|-------------|---------|
| `--header` | Custom HTTP header | `--header "User-Agent: Custom"` |
| `--post-body` | POST request body | `--post-body '{"key":"value"}'` |
| `--single-ip` | Test single IP | `--single-ip 35.207.76.249` |
| `--https` | Use HTTPS (TLS) | `--https` |

## 💡 Practical Tips

### 1. Start with Small Ranges

```bash
# Test with /24 first (256 IPs)
octointel example.com --ranges 35.207.76.0/24

# Then expand to /20 (4096 IPs)
octointel example.com --ranges 35.207.0.0/20

# Finally scan /16 if needed (65536 IPs)
octointel example.com --ranges 35.207.0.0/16
```

### 2. Use Content Matching for Accuracy

```bash
# Instead of just status code...
octointel example.com --status-code 200

# Add content verification to reduce false positives
octointel example.com \
  --method GET \
  --status-code 200 \
  --content-match "<title>Your Unique Title</title>"
```

### 3. Combine Multiple Search Criteria

```bash
# Find backend with specific characteristics
octointel example.com \
  --method GET \
  --status-code 200 \
  --content-match "nginx.*Your-App-Name" \
  --header "Host: example.com"
```

### 4. Save Time with Known Information

If you know your hosting provider:

```bash
# Google Cloud only
octointel example.com --ip-file google-cloud.txt

# AWS only  
octointel example.com --ip-file aws.txt

# Mix of providers
cat google-cloud.txt aws.txt digitalocean.txt > mixed.txt
octointel example.com --ip-file mixed.txt
```

## 🔍 Debugging Tips

### Verbose Mode

```bash
# See every connection attempt
octointel example.com --ranges 35.207.76.0/24 --verbose
```

Output shows:
- Connection attempts
- Failures and reasons
- Response details
- Timing information

### Test Single IP First

```bash
# Verify your pattern works on known IP
octointel example.com --single-ip 35.207.76.249 --verbose
```

### Check Your Regex Pattern

Test regex before scanning large ranges:
```bash
# Test on small range first
octointel example.com \
  --ranges 35.207.76.0/28 \
  --content-match "your-pattern" \
  --verbose
```

## 📊 Understanding Output

```
============================================================
🔍 OctoIntel v1.0.0
⚡ Ultra-fast reverse proxy backend scanner
============================================================

ℹ Auto-detected system capabilities:
  → CPU Cores: 8
  → RAM: 16 GB
  → Concurrent Workers: 5000
  → Connection Timeout: 500ms

============================================================
⚙ Scan Configuration:
============================================================
  → Target domain: example.com
  → HTTP method: GET
  → Port: 80
  → Target status: 200
  → Content match: <title>Example</title>
  → IP ranges: 3
  → Concurrent workers: 5000
  → Timeout: 500ms

============================================================
➤ Scanning 65536 IPs in range 35.207.0.0/16
============================================================
[00:00:15] [████████████████] 65536/65536 (100%) | 4369 IPs/sec

✓ FOUND: 35.207.76.249 - Status: 200, Content matched

⚠ Backend IP found! Stopping scan immediately...

============================================================
✓ Scan completed in 15.23s
============================================================
✓ Found 1 backend IP(s):
  → 35.207.76.249 - Status: 200, Content matched
```

## 🛠️ Troubleshooting

### No Results Found?

1. **Try GET instead of HEAD**
   ```bash
   --method GET
   ```

2. **Change status code**
   ```bash
   --status-code 200  # Try 200 instead of 202
   ```

3. **Remove content matching temporarily**
   ```bash
   # Test without pattern first
   octointel example.com --ranges 35.207.76.0/24
   ```

4. **Use verbose mode**
   ```bash
   -v
   ```

### Scan Too Slow?

```bash
# Reduce workers
--workers 2000

# Increase timeout
--timeout 1000

# Use HEAD instead of GET
--method HEAD
```

### Too Many Open Files (Linux/Mac)

```bash
ulimit -n 65536
```

## 📚 Common Cloud Provider Ranges

### Google Cloud Platform

```text
# North America
35.207.0.0/16
35.208.0.0/16
35.209.0.0/16
35.210.0.0/16

# Europe
35.212.0.0/16
35.213.0.0/16

# Asia
35.215.0.0/16
35.216.0.0/16
```

### Amazon AWS

```text
# US East
3.208.0.0/12
52.0.0.0/14

# US West
13.56.0.0/16
54.176.0.0/12
```

### Digital Ocean

```text
# New York
159.65.0.0/16
167.99.0.0/16

# San Francisco
159.89.0.0/16
165.227.0.0/16
```

## ⚠️ Legal Notice

This tool is provided for **educational and authorized testing purposes only**.

- ✅ Use on your own infrastructure
- ✅ Use with explicit written permission
- ✅ Use for authorized security assessments
- ❌ Do NOT use for unauthorized access
- ❌ Do NOT use for malicious purposes

**Users are responsible for complying with all applicable laws and regulations.**

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Tokio](https://tokio.rs/) async runtime
- CLI powered by [Clap](https://github.com/clap-rs/clap)
- Progress bars by [Indicatif](https://github.com/console-rs/indicatif)

## 📧 Contact

- GitHub: [@Octolus](https://github.com/Octolus)
- Repository: [OctoIntel](https://github.com/Octolus/OctoIntel)
- Website: [OctoVPN](https://octovpn.com)

---

**Made with ❤️ and ⚡ Rust by Octolus from OctoVPN team**
