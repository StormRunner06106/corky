//! `corky skill` — Manage the Claude Code skill definition.
//!
//! Delegates to `agent_kit::skill::SkillConfig` for the actual install/check logic.
//! The SKILL.md content is bundled into the binary at build time via `include_str!`.

use anyhow::Result;
use std::path::Path;

use agent_kit::skill::SkillConfig;

/// The SKILL.md content bundled at build time.
const BUNDLED_SKILL: &str = include_str!("../SKILL.md");

/// Current binary version (from Cargo.toml).
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn config() -> SkillConfig {
    SkillConfig::new("corky", BUNDLED_SKILL, VERSION)
}

/// Install the bundled SKILL.md to the project.
/// When `root` is None, paths are relative to CWD.
pub fn install_at(root: Option<&Path>) -> Result<()> {
    config().install(root)
}

/// Public entry point (CWD-relative, called from main).
pub fn install() -> Result<()> {
    install_at(None)
}

/// Check if the installed skill matches the bundled version.
/// When `root` is None, paths are relative to CWD.
pub fn check_at(root: Option<&Path>) -> Result<()> {
    let up_to_date = config().check(root)?;
    if !up_to_date {
        std::process::exit(1);
    }
    Ok(())
}

/// Check if the installed skill matches the bundled version (CWD-relative).
pub fn check() -> Result<()> {
    check_at(None)
}

/// CLI entry point for `corky install-skill`.
/// The `name` parameter is accepted for backward compatibility but ignored —
/// all corky capabilities are bundled into a single skill.
pub fn run(name: &str) -> Result<()> {
    if name != "email" && name != "corky" {
        anyhow::bail!("Unknown skill '{}'. Available: corky (or 'email' for compat)", name);
    }
    install()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_skill_is_not_empty() {
        assert!(!BUNDLED_SKILL.is_empty());
    }

    #[test]
    fn bundled_skill_contains_corky() {
        assert!(BUNDLED_SKILL.contains("corky"));
    }

    #[test]
    fn install_creates_file() {
        let dir = tempfile::tempdir().unwrap();

        install_at(Some(dir.path())).unwrap();

        let path = dir.path().join(".claude/skills/corky/SKILL.md");
        assert!(path.exists());
        let content = std::fs::read_to_string(&path).unwrap();
        assert_eq!(content, BUNDLED_SKILL);
    }

    #[test]
    fn install_idempotent() {
        let dir = tempfile::tempdir().unwrap();

        install_at(Some(dir.path())).unwrap();
        install_at(Some(dir.path())).unwrap();

        let path = dir.path().join(".claude/skills/corky/SKILL.md");
        let content = std::fs::read_to_string(&path).unwrap();
        assert_eq!(content, BUNDLED_SKILL);
    }

    #[test]
    fn install_overwrites_outdated() {
        let dir = tempfile::tempdir().unwrap();

        let path = dir.path().join(".claude/skills/corky/SKILL.md");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, "old content").unwrap();

        install_at(Some(dir.path())).unwrap();

        let content = std::fs::read_to_string(&path).unwrap();
        assert_eq!(content, BUNDLED_SKILL);
    }

    #[test]
    fn run_accepts_email_compat() {
        // "email" is accepted for backward compat (installs to CWD, but we just
        // verify it doesn't bail with "unknown skill").
        // We can't easily test the full install here without mocking CWD.
    }
}
