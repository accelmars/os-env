# accelmars-os-env

**Read the AccelMars workspace environment from Rust.**

[![License: Apache 2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

`accelmars-os-env` is a tiny shared crate for AccelMars engines.

When the AccelMars OS launches an engine, it passes the resolved workspace location through environment variables.

This crate gives you typed Rust access to those values.

If you are building an AccelMars engine, use this crate.

If you are not, you probably do not need it.

---

## Quick start

Add the dependency:

```toml
[dependencies]
accelmars-os-env = { git = "https://github.com/accelmars/os-env", tag = "accelmars-os-env-v0.3.2" }
```

Then:

```rust
use accelmars_os_env::read_from_env;

fn main() {
    let env = read_from_env()
        .expect("could not read AccelMars workspace environment");

    println!("tenant root: {}", env.tenant_root.display());
    println!("engine home: {}", env.engine_home.display());
}
```

That's the normal production path.

---

# Local development

When you run an engine directly with:

```text
cargo run
```

the AccelMars OS may not be there to set the environment variables.

Use the standalone fallback:

```rust
use accelmars_os_env::{fallback_standalone, read_from_env};

fn main() {
    let env = read_from_env().or_else(|_| {
        let cwd = std::env::current_dir().unwrap();
        fallback_standalone(&cwd)
    })
    .expect("could not locate AccelMars workspace");

    println!("workspace: {}", env.tenant_root.display());
}
```

The fallback walks upward from the current directory looking for `.accelmars/`.

---

# What you get

`read_from_env()` returns:

```rust
pub struct ResolveResult {
    pub tenant_root: PathBuf,
    pub tenant_slug: String,
    pub engine_home: PathBuf,
    pub mode: ResolverMode,
    pub spec_version: u32,
}
```

In normal engine code:

```rust
let env = read_from_env()?;

env.tenant_root;
env.tenant_slug;
env.engine_home;
env.mode;
env.spec_version;
```

---

# The two functions

Most users only need these:

```rust
pub fn read_from_env() -> Result<ResolveResult, EnvError>
```

Read the workspace information supplied by the AccelMars OS.

Use this in production.

And:

```rust
pub fn fallback_standalone(
    cwd: &Path
) -> Result<ResolveResult, EnvError>
```

Find a standalone `.accelmars/` workspace by walking upward from a directory.

Use this for local development when the OS did not launch the process.

---

# Which one should I use?

Usually:

```text
AccelMars OS launched my engine
        ↓
read_from_env()
```

For local development:

```text
cargo run
        ↓
read_from_env()
        ↓ fails because OS env is absent
fallback_standalone()
```

A common setup is therefore:

```rust
let env = read_from_env().or_else(|_| {
    let cwd = std::env::current_dir()?;
    fallback_standalone(&cwd)
})?;
```

---

# Environment variables

The AccelMars OS sets five variables:

| Variable                 | Meaning                             |
| ------------------------ | ----------------------------------- |
| `ACCELMARS_TENANT_ROOT`  | Root directory of the active tenant |
| `ACCELMARS_TENANT_SLUG`  | Active tenant identifier            |
| `ACCELMARS_ENGINE_HOME`  | Directory for the current engine    |
| `ACCELMARS_MODE`         | `standalone` or `integrated`        |
| `ACCELMARS_SPEC_VERSION` | Workspace layout spec version       |

You normally do not need to read these yourself.

Use:

```rust
read_from_env()
```

instead.

The variable-name constants are also exported for tests and tooling.

---

# Modes

There are two resolver modes:

```rust
pub enum ResolverMode {
    Standalone,
    Integrated,
}
```

### Standalone

A single unnamed workspace. The marker on disk is:

```text
.accelmars/
```

but `fallback_standalone()` appends the slug layer, so what you get back is:

```text
tenant_root  = <found>/.accelmars/default
tenant_slug  = "default"
engine_home  = same path as tenant_root
spec_version = 1
```

The slug is `default`, **not** `standalone` — `Standalone` is the *mode*, while `default` is
the tenant name an unnamed workspace gets. Renamed with `os tenant rename default <new-slug>`.

### Integrated

A tenant-specific workspace managed by the AccelMars OS.

For example:

```text
.accelmars/acme/
```

---

# Example

```rust
use accelmars_os_env::{
    fallback_standalone,
    read_from_env,
    ResolverMode,
};

fn main() {
    let result = read_from_env()
        .or_else(|_| {
            let cwd = std::env::current_dir().unwrap();
            fallback_standalone(&cwd)
        })
        .expect("could not locate AccelMars workspace");

    println!("tenant root: {}", result.tenant_root.display());
    println!("engine home: {}", result.engine_home.display());

    match result.mode {
        ResolverMode::Standalone => {
            println!("running standalone");
        }

        ResolverMode::Integrated => {
            println!("tenant: {}", result.tenant_slug);
        }
    }
}
```

---

# Errors

Both resolver functions return:

```rust
pub enum EnvError {
    MissingVar(String),

    InvalidValue {
        var: String,
        value: String,
        reason: String,
    },
}
```

`read_from_env()` fails when required environment variables are missing or invalid.

`fallback_standalone()` fails when it cannot find a valid standalone workspace.

---

# Why does this crate exist?

Without this crate, every AccelMars engine would need to know:

* which environment variables the OS sets;
* how those values are represented;
* how resolver modes are encoded;
* what the current workspace schema looks like.

Worse, engines might need to depend on `anchor` just to get those types.

That would be unnecessary.

Instead:

```text
AccelMars OS
     ↓
resolves workspace
     ↓
sets environment variables
     ↓
engine
     ↓
accelmars-os-env
     ↓
typed ResolveResult
```

This crate is the small contract between the OS and an engine.

---

# Why not depend on Anchor?

`anchor` contains the full workspace resolver.

Engines usually do not need that.

They only need the result.

So the dependency relationship is:

```text
anchor
  │
  │ resolves workspace
  │
  └──────────────┐
                 ▼
        accelmars-os-env
        schema + env reader
                 ▲
                 │
              engine
```

`anchor` may re-export these shared types.

Engines can depend only on this crate.

That keeps the engine dependency surface small.

---

# What this crate does not do

This crate does not implement the full AccelMars workspace resolver.

Do not use it to reproduce workspace-selection logic.

If you need to resolve a workspace yourself, use:

```text
anchor root
```

or depend on the appropriate resolver implementation.

This crate only:

1. reads the workspace environment supplied by the OS; and
2. provides a simple standalone fallback for local development.

---

# Should I use this crate?

## Yes

Use it if you are writing an AccelMars engine that needs to know its workspace.

## Probably not

Do not use it if:

* you are not building an AccelMars engine;
* you need the full workspace resolver;
* you only discovered this repository because `anchor` depends on it.

For most people visiting this repository:

> **You probably want `anchor`, not this crate.**

---

# Installation

This crate is not published to crates.io.

Depend on a Git tag:

```toml
[dependencies]
accelmars-os-env = { git = "https://github.com/accelmars/os-env", tag = "accelmars-os-env-v0.3.2" }
```

Pin a tag.

Do not depend on the default branch.

The workspace schema can change when the AccelMars layout specification changes, and this crate does not promise semver compatibility for external consumers.

Requires Rust 1.70+.

Runtime dependency:

```text
serde
```

No system dependencies.

---

# Why is this repository public?

This crate is public because the public `anchor` project depends on it.

`anchor` can be installed directly from Git:

```bash
cargo install --git https://github.com/accelmars/anchor --tag accelmars-anchor-v2.1.0
```

For that installation to work outside AccelMars, its Git dependencies must also be publicly accessible.

That is why this repository is public.

It is infrastructure, not a separately supported product.

---

# Development

Before pushing, run:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

This repository does not run GitHub Actions.

There will not be a GitHub CI check telling you that the repository passed.

Run the checks locally before opening a pull request.

---

# Telemetry

None.

This crate:

* reads environment variables;
* walks the local filesystem for the development fallback;
* makes no network requests;

---

## License

Apache 2.0 — see [LICENSE](LICENSE). Copyright 2026 AccelMars Co., Ltd.
