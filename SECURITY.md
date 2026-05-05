# Security Policy

## Supported versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | ✅ Yes    |

## Reporting a vulnerability

Please **do not** open a public GitHub issue for security vulnerabilities.

Report vulnerabilities to: **security@accelmars.com**

Include in your report:

- The version of `accelmars-os-env` affected
- A description of the vulnerability and its potential impact
- Reproduction steps or a minimal proof-of-concept
- Any suggested remediation if known

**Response SLA:** We will acknowledge receipt within 48 hours and provide an initial assessment within 5 business days.

## Scope

**In scope:**
- Logic bugs in `read_from_env()` or `fallback_standalone()` that could expose unintended filesystem paths or read incorrect workspace locations
- Environment variable parsing that could be exploited to inject unexpected values

**Out of scope:**
- Vulnerabilities in `serde` or other third-party dependencies — report those upstream
- Infrastructure or deployment issues not related to this crate's code

## PGP

PGP key pending publication.
