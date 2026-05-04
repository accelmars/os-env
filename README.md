# accelmars-resolver-env

Shared types and environment-variable reader for the [AccelMars](https://github.com/accelmars) workspace resolver.

When the AccelMars OS launches an engine process, it serializes the resolved workspace location into a small set of environment variables. This crate defines that schema and provides a typed reader so engine authors can consume it without taking a dependency on [`anchor`](https://github.com/accelmars/anchor) itself.

[![CI](https://github.com/accelmars/resolver-env/actions/workflows/ci.yml/badge.svg)](https://github.com/accelmars/resolver-env/actions/workflows/ci.yml)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

---

## The problem this solves

The AccelMars OS locates a tenant's workspace root before launching any engine (`anchor`, `gateway`, `cortex`, etc.). That location changes depending on whether the install is standalone or multi-tenant. Engines need to know their workspace root at startup, but they should not need to depend on `anchor` — which has its own release cadence, filesystem-walking logic, and heavier dependency tree — just to read a path.

This crate is the answer: a tiny (~50 LOC), `serde`-only library that defines the five resolver fields and provides two functions:

- **`read_from_env()`** — reads all five environment variables set by the OS at engine launch
- **`fallback_standalone(cwd)`** — walks parent directories to find `.accelmars/` for development environments where the OS has not launched the engine

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
accelmars-resolver-env = "0.1"
```

Requires Rust 1.70+. No system dependencies. `serde` is the only runtime dependency.

## Quick start

```rust
use accelmars_resolver_env::{read_from_env, fallback_standalone, ResolverMode};

fn main() {
    // In production: OS sets all five env vars before launching your engine.
    let result = read_from_env()
        .or_else(|_| {
            // Fallback for `cargo run` / local development without the OS.
            let cwd = std::env::current_dir().unwrap();
            fallback_standalone(&cwd)
        })
        .expect("could not locate AccelMars workspace");

    println!("tenant root: {}", result.tenant_root.display());
    println!("engine home: {}", result.engine_home.display());

    match result.mode {
        ResolverMode::Standalone => println!("running in standalone mode"),
        ResolverMode::Integrated => println!("tenant: {}", result.tenant_slug),
    }
}
```

## Environment variable schema

The OS sets these five variables when launching an engine:

| Variable | Type | Description |
|----------|------|-------------|
| `ACCELMARS_TENANT_ROOT` | absolute path | Root directory for the active tenant (`.accelmars/<slug>/` in integrated mode; `.accelmars/` in standalone) |
| `ACCELMARS_TENANT_SLUG` | string | Active tenant identifier; `"standalone"` in standalone mode |
| `ACCELMARS_ENGINE_HOME` | absolute path | This engine's directory under the tenant root |
| `ACCELMARS_MODE` | `"standalone"` \| `"integrated"` | Resolver mode detected by `anchor` |
| `ACCELMARS_SPEC_VERSION` | integer | Layout spec version from `MANIFEST.toml` |

All constants are exported as `ENV_*` pub consts for use in tests and tooling.

## API

```rust
// Read from env vars (production path)
pub fn read_from_env() -> Result<ResolveResult, EnvError>

// Walk parents from cwd looking for .accelmars/ (development fallback)
pub fn fallback_standalone(cwd: &Path) -> Result<ResolveResult, EnvError>

pub struct ResolveResult {
    pub tenant_root: PathBuf,
    pub tenant_slug: String,
    pub engine_home: PathBuf,
    pub mode: ResolverMode,
    pub spec_version: u32,
}

pub enum ResolverMode { Standalone, Integrated }

pub enum EnvError {
    MissingVar(String),
    InvalidValue { var: String, value: String, reason: String },
}
```

## Architecture

This crate sits at the bottom of the AccelMars engine dependency graph. The design constraint is intentional: **no anchor dependency**. Anchor implements `WorkspaceResolver` and returns a `ResolveResult` (re-exported from this crate), but engines never need to link anchor. They link this crate, read the env vars the OS sets, and proceed.

```
anchor (resolver impl) ──re-exports──► accelmars-resolver-env (schema + reader)
                                                ▲
                                    gateway / cortex / pact / ...
                                    (depend only on this crate)
```

## What NOT to use this for

- **Resolving the workspace yourself** — use `anchor root` or depend on `anchor` if you need the full resolver. This crate only reads env vars and does a simple fallback walk; it does not implement the full slug-selection precedence ladder.
- **Server-side tenant identification** — on the server, tenant ID comes from the `X-Anchor-Tenant-ID` HTTP header set by the platform. This crate is for process-local startup only.
- **Anything outside AccelMars engines** — if you are not writing an engine that the AccelMars OS launches, this crate is not for you.

## Telemetry

This crate collects no telemetry. It makes no network calls. It reads environment variables and walks the local filesystem.

## License

Apache 2.0 — see [LICENSE](LICENSE).
Copyright 2026 AccelMars Co., Ltd.
