# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

## [0.1.0] - 2026-05-05

### Features

- `ResolveResult` struct — typed representation of the five resolver fields (`tenant_root`, `tenant_slug`, `engine_home`, `mode`, `spec_version`)
- `ResolverMode` enum — `Standalone` | `Integrated`
- `read_from_env()` — reads all five `ACCELMARS_*` environment variables into a typed `ResolveResult`
- `fallback_standalone(cwd)` — parent-walks from `cwd` to find `.accelmars/` for development environments launched outside the OS
- `EnvError` — typed error with `MissingVar` and `InvalidValue` variants; carries variable name and invalid value for diagnostics
- Exported `ENV_*` constants for all five variable names
