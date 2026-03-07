# Security Policy

## Supported Versions

We actively support security updates for:

| Version | Supported          | End of Support |
| ------- | ------------------ | -------------- |
| 0.2.x   | ✅ Yes             | Current        |
| < 0.2   | ❌ No              | Ended          |

## Reporting a Vulnerability

**CRITICAL: DO NOT create public GitHub issues for security vulnerabilities.**

### Responsible Disclosure

1. **Email**: security@inteleion.ai
2. **PGP Key**: Available at keybase.io/inteleionai
3. **Response Time**: Within 48 hours
4. **Status Updates**: Every 3-5 business days

### What to Include

```
Subject: [SECURITY] Brief description

1. Vulnerability description
2. Steps to reproduce
3. Affected versions
4. Potential impact (CVSS score if known)
5. Suggested fix (optional)
6. Discovery credit (if desired)
```

### Our Commitment

- **Acknowledgment**: Within 48 hours
- **Initial Assessment**: Within 5 business days
- **Fix Timeline**:
  - Critical (CVSS 9.0-10.0): 24-72 hours
  - High (CVSS 7.0-8.9): 1 week
  - Medium (CVSS 4.0-6.9): 2 weeks
  - Low (CVSS 0.1-3.9): Next release

## Security Best Practices

### For Users

#### Input Validation
```python
# Always validate embedding dimensions
if embedding.len() != expected_dimension:
    return Err(Error::DimensionMismatch { ... })

# Sanitize route IDs
route_id = route_id.trim()
if route_id.contains("..") or route_id.contains("/"):
    return Err(Error::InvalidInput("Invalid route ID"))
```

#### Rate Limiting
Implement rate limiting in production to prevent abuse.

#### Memory Limits
Monitor memory usage:
- ~64MB per 1000 routes
- Scale linearly
- Set ulimits in production

### For Developers

1. **Never use `unsafe` without audit**
2. **Validate all external input**
3. **Run `cargo audit` before commits**
4. **Use `cargo deny` for license checks**
5. **Add security tests for new features**

## Vulnerability Disclosure

We follow coordinated disclosure:
1. Private notification to affected parties
2. Fix developed and tested
3. Security advisory published
4. CVE assigned if applicable

## Security Audit Log

### v0.2.0 (2024-12-21)
- Initial security review completed
- No vulnerabilities identified
- Added input validation
- Added rate limiting guidance

## Contact

- Security: security@inteleion.ai
- General: hello@inteleion.ai
