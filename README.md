# Vulnerable Rust/Actix-Web Application

**⚠️ WARNING: This application is intentionally vulnerable and should NEVER be deployed to production!**

This is an intentionally vulnerable Rust/Actix-web application designed for testing security remediation tools, SAST/SCA scanners, and security training.

## 🎯 Purpose

- **200+ open source dependency vulnerabilities** requiring complex remediation
- **20+ code-level security vulnerabilities** covering OWASP Top 10
- Realistic vulnerable code patterns found in real-world Rust applications
- Various remediation scenarios (simple upgrades, breaking changes, deprecated crates)

## 📊 Vulnerability Summary

### Dependency Vulnerabilities (SCA)
- **30 vulnerable Rust crates** from 2019
- actix-web 1.0, tokio 0.1.22, openssl 0.10.24 with known vulnerabilities
- Expected **200+ total vulnerabilities** across direct and transitive dependencies

**Key Vulnerable Crates:**
- `actix-web: 1.0` (multiple security issues)
- `tokio: 0.1.22` (memory safety issues)
- `openssl: 0.10.24` (cryptographic vulnerabilities)
- `reqwest: 0.9.19` (SSRF, header injection)
- `diesel: 1.4.4` (SQL injection patterns)
- `regex: 1.1.9` (ReDoS vulnerabilities)
- `yaml-rust: 0.4.3` (deserialization issues)
- And 23+ more...

### Code Vulnerabilities (SAST) - 20 Vulnerabilities

1. **SQL Injection** (CWE-89) - main.rs:73
2. **Command Injection** (CWE-78) - main.rs:83-91
3. **Path Traversal** (CWE-22) - main.rs:99-107
4. **XSS** (CWE-79) - main.rs:114-119
5. **SSRF** (CWE-918) - main.rs:124-134
6. **Unsafe Code Patterns** (CWE-94) - main.rs:137-143
7. **Mass Assignment** (CWE-915) - main.rs:148-162
8. **IDOR** (CWE-639) - main.rs:167-175
9. **Missing Authentication** (CWE-306) - main.rs:180-190
10. **Sensitive Data Exposure** (CWE-200) - main.rs:195-202
11. **Open Redirect** (CWE-601) - main.rs:207-215
12. **Weak Cryptography** (CWE-327) - main.rs:222-232
13. **ReDoS** (CWE-1333) - main.rs:237-246
14. **Insecure Randomness** (CWE-330) - main.rs:251-260
15. **Hardcoded Credentials** (CWE-798) - main.rs:14-17, 265-275
16. **Information Exposure via Errors** (CWE-209) - main.rs:280-288
17. **Missing Rate Limiting** (CWE-770) - main.rs:293-300
18. **Cleartext Transmission** (CWE-319) - main.rs:307-314
19. **Sensitive Data in GET** (CWE-598) - main.rs:319-330
20. **Exposed Secrets** - .env file

## 🚀 Setup

### Prerequisites
- Rust 1.40+ (2019 edition)
- Cargo

### Installation

```bash
git clone https://github.com/YOUR_USERNAME/vulnerable-rust-app.git
cd vulnerable-rust-app

# Build (expect warnings about vulnerabilities)
cargo build

# Run
cargo run
```

Access at: `http://localhost:8080`

## 🔍 Testing Vulnerabilities

### Scan with Snyk
```bash
snyk test
```

### Scan with cargo-audit
```bash
cargo install cargo-audit
cargo audit
```

### Expected Results: 200+ vulnerabilities

## 📚 Available Vulnerable Endpoints

- `POST /api/login` - SQL Injection
- `GET /api/exec?cmd=ls` - Command Injection
- `GET /api/files?filename=test.txt` - Path Traversal
- `GET /api/search?query=test` - XSS
- `GET /api/proxy?url=http://example.com` - SSRF
- `POST /api/eval` - Unsafe Code Execution
- `POST /api/register` - Mass Assignment
- `GET /api/users/{id}` - IDOR
- `DELETE /api/admin/users/{id}` - Missing Authentication
- `GET /api/debug` - Sensitive Data Exposure
- `GET /api/redirect?url=evil.com` - Open Redirect
- `POST /api/hash` - Weak Cryptography
- `GET /api/regex?input=aaa!` - ReDoS
- `GET /api/generate-token` - Insecure Randomness
- And 5 more...

## 🛡️ Remediation Scenarios

### Simple Direct Upgrades
```bash
cargo update actix-web
cargo update tokio
```

### Major Version Upgrades (Breaking Changes)
```bash
# actix-web 1.0 → 4.0
# Requires significant code refactoring
```

### Deprecated Crate Migration
- Some old crates no longer maintained
- May require alternative solutions

## ⚠️ Security Notice

**DO NOT:**
- Deploy to production
- Expose to the internet
- Use code patterns in real applications
- Commit .env to version control

**DO:**
- Use for security testing
- Use for security training
- Run in isolated environments

## ⚡ Quick Start

```bash
cargo build
cargo run

# In another terminal:
curl http://localhost:8080/api/debug
```

---

**Remember**: Intentionally insecure. Use responsibly in controlled environments only!

MIT License - Educational and testing purposes only.
