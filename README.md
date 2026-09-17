# accelmars-os-env

**Read the AccelMars workspace environment from Rust.**

[![License: Apache 2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

When the AccelMars OS launches an engine, it passes the resolved workspace location through
environment variables. This crate gives you typed access to those values.

If you are building an AccelMars engine, use this crate.
If you are not, you probably want [`anchor`](https://github.com/accelmars/anchor) instead.

> **No CI.** This repository runs no GitHub Actions and shows no status checks. The quality gate
> runs locally on `git push`, so a PR here will never go green — run the checks yourself.

---

## Quick start

```toml
[dependencies]
accelmars-os-env = { git = "https://github.com/accelmars/os-env", tag = "accelmars-os-env-v0.3.2" }
```

```rust
use accelmars_os_env::read_from_env;

fn main() {
    let env = read_from_env()
        .expect("could not read AccelMars workspace environment");

    println!("tenant root: {}", env.tenant_root.display());
    println!("engine home: {}", env.engine_home.display());
}
```

That is the production path: the OS set the variables before your process started.

---

## Local development

Running an engine yourself with `cargo run` means the OS is not there to set anything. Fall back
to walking up from the current directory looking for `.accelmars/`:

```rust
use accelmars_os_env::{fallback_standalone, read_from_env};

fn main() {
    let env = read_from_env()
        .or_else(|_| {
            let cwd = std::env::current_dir().unwrap();
            fallback_standalone(&cwd)
        })
        .expect("could not locate AccelMars workspace");

    println!("workspace: {}", env.tenant_root.display());
}
```

That two-step is the normal shape for an engine that must run both ways.

### What the fallback actually returns

The fallback does not simply hand back the directory it found. Given `.accelmars/` on disk it
returns:

| Field | Value |
|---|---|
| `tenant_root` | `<found>/.accelmars/default` — **the `default/` slug layer is appended** |
| `tenant_slug` | `"default"` |
| `engine_home` | the same path as `tenant_root` |
| `mode` | `ResolverMode::Standalone` |
| `spec_version` | `1`, hardcoded |

Two things surprise people. The slug is `"default"`, not `"standalone"` — `Standalone` is the
*mode*, while `default` is the tenant name an unnamed workspace gets (`STANDALONE_SLUG`, renamed
via `os tenant rename default <new-slug>`). And in this path `engine_home` equals `tenant_root`,
because there is no OS to assign the engine its own directory; do not rely on them differing.

### Bounded walk

`fallback_standalone` walks upward until it hits the filesystem root, so under a temp directory it
can escape and match a real `.accelmars/` above. For tests and sandboxes, stop it:

```rust
fallback_standalone_bounded(&cwd, Some(&sandbox_root))?;
```

`stop_at = None` behaves exactly like `fallback_standalone`.

---

## What you get

```rust
pub struct ResolveResult {
    pub tenant_root: PathBuf,
    pub tenant_slug: String,
    pub engine_home: PathBuf,
    pub mode: ResolverMode,
    pub spec_version: u32,
}

pub enum ResolverMode {
    Standalone,   // one unnamed workspace; slug "default"
    Integrated,   // a tenant the AccelMars OS manages, e.g. .accelmars/acme/
}
```

---

## Environment variables

The OS sets five. `read_from_env()` requires **all** of them — a missing one is an error, not a
default.

| Variable | Meaning |
|---|---|
| `ACCELMARS_TENANT_ROOT` | root directory of the active tenant |
| `ACCELMARS_TENANT_SLUG` | active tenant identifier |
| `ACCELMARS_ENGINE_HOME` | directory for the current engine |
| `ACCELMARS_MODE` | `standalone` or `integrated` |
| `ACCELMARS_SPEC_VERSION` | workspace layout spec version (a `u32`) |

Read them through `read_from_env()` rather than by hand. The names are also exported as
`ENV_TENANT_ROOT`, `ENV_TENANT_SLUG`, `ENV_ENGINE_HOME`, `ENV_MODE` and `ENV_SPEC_VERSION` for
tests and tooling.

---

## Errors

```rust
pub enum EnvError {
    MissingVar(String),
    InvalidValue { var: String, value: String, reason: String },
}
```

`read_from_env()` returns `MissingVar` for an absent variable, and `InvalidValue` when `MODE` is
not one of the two accepted strings or `SPEC_VERSION` does not parse as a `u32`.

**A wart worth knowing:** when `fallback_standalone` finds no `.accelmars/` anywhere above `cwd`,
it also reports `MissingVar("ACCELMARS_TENANT_ROOT")`. Nothing was looking at an environment
variable at that point — the message reads as though something was. Treat that specific error from
the fallback as *"no workspace found"*, and say so in your own message to the user.

---

## Why this crate exists

Without it, every engine would have to know which variables the OS sets, how the values are
encoded, and what the current layout spec looks like — or depend on `anchor` just to borrow the
types.

```text
AccelMars OS  →  resolves workspace  →  sets env vars
                                             ↓
                                          engine
                                             ↓
                                     accelmars-os-env
                                             ↓
                                      ResolveResult
```

`anchor` holds the full resolver; engines only need the result. So `anchor` depends on this crate,
and an engine can depend on this crate alone and stay small.

## What it does not do

It does not implement the workspace resolver. Do not use it to reproduce workspace-selection
logic — that lives in `anchor` (`anchor root`). This crate reads what the OS supplied, and offers
one simple upward-walking fallback for development.

## Should you use it?

**Yes**, if you are writing an AccelMars engine that needs to know its workspace.

**Probably not**, if you are not building one, if you need the full resolver, or if you only found
this repository because `anchor` depends on it. In that last case:

> You almost certainly want [`anchor`](https://github.com/accelmars/anchor), not this crate.

---

## Why this repository is public

Because [`anchor`](https://github.com/accelmars/anchor) is public, Apache-2.0, and installable
from source — and it depends on this crate by git URL:

```sh
cargo install --git https://github.com/accelmars/anchor --tag accelmars-anchor-v2.1.0
```

If this repository were private, that command would fail for everyone outside AccelMars. It is
here so `anchor` works. It is infrastructure, not a separately supported product.

---

## Installation notes

Not published to crates.io, and none is planned. **Pin a tag; do not track the default branch.**
The schema changes when the AccelMars layout spec does, and this crate makes no semver promise to
outside consumers.

Requires Rust 1.70+ (declared as `rust-version`, so cargo enforces it). `serde` is the only
runtime dependency. No system dependencies.

---

## Development

```sh
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

Run these before opening a PR — nothing on GitHub will run them for you. See
[CONTRIBUTING.md](CONTRIBUTING.md).

---

## Telemetry

None. This crate reads environment variables and walks the local filesystem for the development
fallback. It makes no network requests, and nothing leaves your machine.

---

## License

Apache 2.0 — see [LICENSE](LICENSE). Copyright 2026 AccelMars Co., Ltd.
