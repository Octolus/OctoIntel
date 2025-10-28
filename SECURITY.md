# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 2.0.x   | :white_check_mark: |
| < 2.0   | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability in OctoIntel, please follow these steps:

### 1. Do Not Open a Public Issue

Please **DO NOT** create a public GitHub issue for security vulnerabilities. This helps prevent malicious actors from exploiting the vulnerability before a fix is available.

### 2. Report Privately

Send a detailed report to: **[Your Email or Security Contact]**

Include in your report:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if you have one)
- Your contact information for follow-up

### 3. Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Fix Timeline**: Depends on severity
  - Critical: 1-7 days
  - High: 7-14 days
  - Medium: 14-30 days
  - Low: 30-90 days

### 4. Disclosure Policy

- We will confirm receipt of your report within 48 hours
- We will provide a detailed response within 7 days
- We will keep you informed of our progress
- We will credit you in the security advisory (unless you prefer to remain anonymous)
- We will coordinate public disclosure timing with you

## Security Best Practices for Users

When using OctoIntel:

1. **Legal Use Only**
   - Only scan infrastructure you own or have explicit permission to test
   - Comply with all applicable laws and regulations
   - Respect rate limits and terms of service

2. **Data Privacy**
   - Be careful with verbose output (`--verbose`) as it may contain sensitive information
   - Don't share scan results publicly without permission
   - Secure any logs or output files

3. **Network Security**
   - Be aware that high-volume scanning may trigger security systems
   - Use appropriate worker counts to avoid overwhelming targets
   - Consider network segmentation when scanning

4. **Keep Updated**
   - Use the latest version of OctoIntel
   - Monitor for security updates
   - Review CHANGELOG for security-related fixes

## Known Security Considerations

### 1. Rate Limiting

High-speed scanning may:
- Trigger IDS/IPS systems
- Cause unintentional DoS on targets
- Get your IP blocked or flagged

**Mitigation**: Use appropriate `--workers` and `--timeout` values.

### 2. Information Disclosure

The tool may reveal:
- Backend infrastructure details
- IP address mappings
- Server configurations

**Mitigation**: Only use on authorized targets.

### 3. Network Exposure

Aggressive scanning:
- Generates significant network traffic
- May be logged by intermediary systems
- Could attract unwanted attention

**Mitigation**: Use responsibly and ethically.

## Security Features

### What We Do

- ✅ Input validation for all user inputs
- ✅ Safe Rust practices (no unsafe code)
- ✅ Dependency security scanning
- ✅ Minimal external dependencies
- ✅ No data collection or telemetry
- ✅ Open source and auditable

### What We Don't Do

- ❌ Store scan results
- ❌ Phone home or send telemetry
- ❌ Include backdoors or malicious code
- ❌ Bypass security controls

## Dependencies

We regularly audit our dependencies for security vulnerabilities:

```bash
cargo audit
```

Critical dependencies:
- `tokio` - Async runtime (actively maintained)
- `clap` - CLI parsing (actively maintained)
- `regex` - Pattern matching (actively maintained)

## Vulnerability Disclosure Timeline

When we discover or are informed of a vulnerability:

1. **Day 0**: Vulnerability reported/discovered
2. **Day 1-2**: Confirm and assess severity
3. **Day 3-7**: Develop and test fix
4. **Day 7-14**: Release patched version
5. **Day 14+**: Public disclosure with credit

## Bug Bounty Program

Currently, we do not offer a bug bounty program. However, we greatly appreciate security researchers who report vulnerabilities responsibly and will:

- Credit you in security advisories
- Recognize your contribution in CHANGELOG
- Provide acknowledgment in README (if desired)

## Hall of Fame

Security researchers who have helped make OctoIntel more secure:

- *Your name could be here!*

## Contact

For security-related inquiries:
- **Organization**: OctoVPN
- **GitHub**: [@Octolus](https://github.com/Octolus)
- **Repository**: [Issues](https://github.com/Octolus/OctoIntel/issues) (for non-security bugs only)

## Legal

This project is provided "as is" under the MIT License. Users are responsible for ensuring their use of this tool complies with all applicable laws and regulations.

