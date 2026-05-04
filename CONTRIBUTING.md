# Contributing to accelmars-resolver-env

Thank you for your interest. This is a small, intentionally stable crate — its primary constraint is that it must remain free of the `anchor` dependency.

## Prerequisites

- Rust stable (1.70+)
- `cargo` in `$PATH`
- No system dependencies

## Setup

```bash
git clone https://github.com/accelmars/resolver-env
cd resolver-env
cargo build
cargo test
```

Four commands, zero to green.

## Contribution types accepted

- Bug fixes in `read_from_env()` or `fallback_standalone()`
- Documentation improvements (README, rustdoc)
- Test coverage additions
- New env var fields, if the AccelMars resolver schema is extended (coordinate via issue first)

**Not accepted:**
- Dependencies beyond `serde` — the no-anchor, minimal-dep contract is a hard constraint
- Breaking changes to `ResolveResult` or `ResolverMode` without a major version bump
- Additions that make assumptions about the AccelMars platform outside this crate's stated scope

## Commit format

[Conventional Commits](https://www.conventionalcommits.org/) required on all commits:

```
feat: add engine_home field to ResolveResult
fix: handle empty string in ACCELMARS_TENANT_SLUG
docs: clarify fallback_standalone cwd semantics
```

- `feat:` — new capability
- `fix:` — bug fix
- `docs:` — documentation only
- `chore:` — tooling, CI, formatting
- `refactor:` — internal restructure, no behavior change

**Never include** internal methodology strings, contract IDs, or guild workflow language in commit titles. Commit titles are read by external contributors in release notes.

## Branching

All changes via PR. Branch naming conventions:

| Prefix | Use for |
|--------|---------|
| `feat/` | New capabilities |
| `fix/` | Bug fixes |
| `docs/` | Documentation only |
| `chore/` | Tooling, CI, formatting |
| `refactor/` | Internal restructure |

## PR process

1. Create a branch from `main` with the appropriate prefix
2. Make your changes; ensure `cargo fmt`, `cargo clippy -- -D warnings`, and `cargo test` all pass
3. Open a PR — use the PR template
4. CI must pass before merge
5. Squash merge only — one commit per PR on `main`

## What NOT to commit

- `.env` files, API keys, tokens, or credentials of any kind
- Hardcoded absolute paths (e.g. `/Users/yourname/...`)
- Internal platform identifiers or private repository names
