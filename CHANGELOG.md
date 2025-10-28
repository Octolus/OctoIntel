# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2025-10-28

### Added

- 🚀 **Production-ready release** with comprehensive features
- 📁 **File-based IP range loading** - Load IPs from text files with CIDR notation
- 🌐 **Multiple HTTP methods** - Support for HEAD, GET, and POST requests
- 🔍 **Content matching** - Flexible regex-based HTML content search
- 🎯 **Custom status codes** - Match any HTTP status code (not just 202)
- 🔧 **Custom HTTP headers** - Add multiple custom headers to requests
- 📝 **POST body support** - Send POST data with requests
- 🔌 **Custom port support** - Scan on any port (default: 80)
- 💬 **Verbose mode** - Detailed debug output for troubleshooting
- ⚙️ **Auto-optimization** - Dynamic CPU and RAM detection for optimal performance
- 📊 **Enhanced output** - Beautiful colored output with detailed information
- 🛡️ **Production error handling** - Comprehensive validation and error messages
- 📚 **Comprehensive documentation** - Detailed README with examples
- 🤝 **Contributing guidelines** - CONTRIBUTING.md for contributors
- 🔒 **Security policy** - SECURITY.md for responsible disclosure
- ⚖️ **MIT License** - Open source license
- 📝 **Example files** - ips.txt.example with cloud provider ranges
- 🔄 **CI/CD pipeline** - GitHub Actions for automated testing and builds
- 🎨 **Better CLI** - Improved help text and argument parsing

### Changed

- 🏗️ **Project renamed** - From `ip_scanner` to `reverse-proxy-reveal`
- 📦 **Binary renamed** - Now `reverse-proxy-reveal` instead of `ip_scanner`
- 🎯 **Improved scanning logic** - Better buffer management for content matching
- 📈 **Performance optimizations** - Dynamic buffer sizing based on scan type
- 🎨 **Enhanced UI** - Better progress bars and colored output
- 📖 **Documentation overhaul** - Complete README rewrite with detailed examples

### Fixed

- ✅ **Error handling** - Better validation of user inputs
- 🐛 **Regex compilation** - Validates regex patterns before scanning
- 🔍 **Content matching** - More reliable HTML content detection
- 🚫 **Input validation** - Validates CIDR notation and IP addresses

### Security

- 🔒 Added security policy and disclosure guidelines
- ✅ Input validation for all user-provided data
- 🛡️ Safe Rust practices throughout codebase
- 📋 Security considerations documented

## [1.0.0] - 2025-10-27

### Added

- Initial release
- Basic IP scanning with HEAD requests
- Support for CIDR notation
- Progress bar with ETA
- Stop-on-find feature
- Auto-detected worker count and timeout
- Support for single IP scanning
- Google Cloud default ranges

---

## Unreleased

Nothing yet!

---

[2.0.0]: https://github.com/Octolus/OctoIntel/releases/tag/v2.0.0
[1.0.0]: https://github.com/Octolus/OctoIntel/releases/tag/v1.0.0

