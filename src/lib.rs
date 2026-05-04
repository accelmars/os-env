use std::path::{Path, PathBuf};

// Env var name constants
pub const ENV_TENANT_ROOT: &str = "ACCELMARS_TENANT_ROOT";
pub const ENV_TENANT_SLUG: &str = "ACCELMARS_TENANT_SLUG";
pub const ENV_ENGINE_HOME: &str = "ACCELMARS_ENGINE_HOME";
pub const ENV_MODE: &str = "ACCELMARS_MODE";
pub const ENV_SPEC_VERSION: &str = "ACCELMARS_SPEC_VERSION";

#[derive(Debug, PartialEq, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResolverMode {
    Standalone,
    Integrated,
}

#[derive(Debug, PartialEq)]
pub struct ResolveResult {
    pub tenant_root: PathBuf,
    pub tenant_slug: String,
    pub engine_home: PathBuf,
    pub mode: ResolverMode,
    pub spec_version: u32,
}

#[derive(Debug, PartialEq)]
pub enum EnvError {
    MissingVar(String),
    InvalidValue {
        var: String,
        value: String,
        reason: String,
    },
}

impl std::fmt::Display for EnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvError::MissingVar(v) => write!(f, "missing env var: {}", v),
            EnvError::InvalidValue { var, value, reason } => {
                write!(f, "invalid value for {}: {:?} — {}", var, value, reason)
            }
        }
    }
}

/// Read the AccelMars workspace resolver result from environment variables.
/// Pure env-var reads — does not touch the filesystem.
pub fn read_from_env() -> Result<ResolveResult, EnvError> {
    let tenant_root = PathBuf::from(require_var(ENV_TENANT_ROOT)?);
    let tenant_slug = require_var(ENV_TENANT_SLUG)?;
    let engine_home = PathBuf::from(require_var(ENV_ENGINE_HOME)?);

    let mode_str = require_var(ENV_MODE)?;
    let mode = match mode_str.as_str() {
        "standalone" => ResolverMode::Standalone,
        "integrated" => ResolverMode::Integrated,
        _ => {
            return Err(EnvError::InvalidValue {
                var: ENV_MODE.to_string(),
                value: mode_str,
                reason: "expected \"standalone\" or \"integrated\"".to_string(),
            })
        }
    };

    let version_str = require_var(ENV_SPEC_VERSION)?;
    let spec_version = version_str.parse::<u32>().map_err(|_| EnvError::InvalidValue {
        var: ENV_SPEC_VERSION.to_string(),
        value: version_str,
        reason: "expected a non-negative integer".to_string(),
    })?;

    Ok(ResolveResult {
        tenant_root,
        tenant_slug,
        engine_home,
        mode,
        spec_version,
    })
}

fn require_var(name: &str) -> Result<String, EnvError> {
    std::env::var(name).map_err(|_| EnvError::MissingVar(name.to_string()))
}

/// Locate the workspace root by walking parents from `cwd` looking for `.accelmars/`.
/// Constructs a standalone `ResolveResult` — does not read env vars.
pub fn fallback_standalone(cwd: &Path) -> Result<ResolveResult, EnvError> {
    let mut current = cwd.to_path_buf();
    loop {
        let marker = current.join(".accelmars");
        if marker.is_dir() {
            let tenant_root = marker;
            return Ok(ResolveResult {
                engine_home: tenant_root.clone(),
                tenant_root,
                tenant_slug: "standalone".to_string(),
                mode: ResolverMode::Standalone,
                spec_version: 1,
            });
        }
        match current.parent().map(|p| p.to_path_buf()) {
            Some(p) if p != current => current = p,
            _ => {
                return Err(EnvError::MissingVar(ENV_TENANT_ROOT.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::sync::Mutex;

    // Serialize env-mutating tests to avoid interference between parallel test runs.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn set_all_vars() {
        env::set_var(ENV_TENANT_ROOT, "/tmp/.accelmars/AOS");
        env::set_var(ENV_TENANT_SLUG, "AOS");
        env::set_var(ENV_ENGINE_HOME, "/tmp/.accelmars/AOS/gateway");
        env::set_var(ENV_MODE, "integrated");
        env::set_var(ENV_SPEC_VERSION, "1");
    }

    fn clear_all_vars() {
        env::remove_var(ENV_TENANT_ROOT);
        env::remove_var(ENV_TENANT_SLUG);
        env::remove_var(ENV_ENGINE_HOME);
        env::remove_var(ENV_MODE);
        env::remove_var(ENV_SPEC_VERSION);
    }

    #[test]
    fn read_from_env_happy_path() {
        let _lock = ENV_LOCK.lock().unwrap();
        set_all_vars();
        let result = read_from_env().expect("should succeed");
        assert_eq!(result.tenant_root, PathBuf::from("/tmp/.accelmars/AOS"));
        assert_eq!(result.tenant_slug, "AOS");
        assert_eq!(result.engine_home, PathBuf::from("/tmp/.accelmars/AOS/gateway"));
        assert_eq!(result.mode, ResolverMode::Integrated);
        assert_eq!(result.spec_version, 1);
        clear_all_vars();
    }

    #[test]
    fn read_from_env_missing_var() {
        let _lock = ENV_LOCK.lock().unwrap();
        set_all_vars();
        env::remove_var(ENV_TENANT_SLUG);
        let err = read_from_env().expect_err("should fail on missing var");
        assert_eq!(err, EnvError::MissingVar(ENV_TENANT_SLUG.to_string()));
        clear_all_vars();
    }

    #[test]
    fn read_from_env_invalid_mode() {
        let _lock = ENV_LOCK.lock().unwrap();
        set_all_vars();
        env::set_var(ENV_MODE, "bogus");
        let err = read_from_env().expect_err("should fail on invalid mode");
        match err {
            EnvError::InvalidValue { var, .. } => assert_eq!(var, ENV_MODE),
            other => panic!("expected InvalidValue, got {:?}", other),
        }
        clear_all_vars();
    }

    #[test]
    fn read_from_env_invalid_spec_version() {
        let _lock = ENV_LOCK.lock().unwrap();
        set_all_vars();
        env::set_var(ENV_SPEC_VERSION, "abc");
        let err = read_from_env().expect_err("should fail on invalid spec version");
        match err {
            EnvError::InvalidValue { var, .. } => assert_eq!(var, ENV_SPEC_VERSION),
            other => panic!("expected InvalidValue, got {:?}", other),
        }
        clear_all_vars();
    }

    #[test]
    fn fallback_standalone_happy_path() {
        let dir = tempfile::tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join(".accelmars")).unwrap();
        let deep = dir.path().join("a").join("b").join("c");
        fs::create_dir_all(&deep).unwrap();

        let result = fallback_standalone(&deep).expect("should find .accelmars/");
        assert_eq!(result.mode, ResolverMode::Standalone);
        assert_eq!(result.tenant_slug, "standalone");
        assert!(
            result.tenant_root.ends_with(".accelmars"),
            "tenant_root should end with .accelmars, got {:?}",
            result.tenant_root
        );
    }

    #[test]
    fn fallback_standalone_not_found() {
        let dir = tempfile::tempdir().expect("tempdir");
        let err = fallback_standalone(dir.path()).expect_err("should fail — no .accelmars/");
        assert_eq!(err, EnvError::MissingVar(ENV_TENANT_ROOT.to_string()));
    }
}
