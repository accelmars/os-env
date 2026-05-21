# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

## [0.2.0] - 2026-05-21

### Changed

- **`fallback_standalone()` now injects a `default/` slug.** Previously returned `engine_home = .accelmars/` with `tenant_slug = "standalone"` — a no-slug-layer shape that contradicted OS-ARC22 ("every layout is `.accelmars/<slug>/`"). Now returns `tenant_root = .accelmars/default/`, `tenant_slug = "default"`, `engine_home = .accelmars/default/`. `default` matches the world-class convention (Kubernetes namespaces, AWS CLI profiles, Terraform workspaces, Docker Compose project names). Upgrade path is mechanical: `os tenant rename default <new-slug>`.
- `ResolveResult` and `ResolverMode` now derive `Clone + Eq` (in addition to `Debug + PartialEq`). Required by consumers that need to embed `ResolveResult` in a longer-lived context and cheaply hand it to per-task sub-contexts (e.g., pact-engine's `PactSlot`).

### Added

- Exported `STANDALONE_SLUG` const (= `"default"`) so consumers reference the canonical name without hardcoding the string.

## [0.1.0] - 2026-05-05

### Features

- `ResolveResult` struct — typed representation of the five resolver fields (`tenant_root`, `tenant_slug`, `engine_home`, `mode`, `spec_version`)
- `ResolverMode` enum — `Standalone` | `Integrated`
- `read_from_env()` — reads all five `ACCELMARS_*` environment variables into a typed `ResolveResult`
- `fallback_standalone(cwd)` — parent-walks from `cwd` to find `.accelmars/` for development environments launched outside the OS
- `EnvError` — typed error with `MissingVar` and `InvalidValue` variants; carries variable name and invalid value for diagnostics
- Exported `ENV_*` constants for all five variable names
