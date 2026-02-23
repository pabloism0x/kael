use std::path::{Path, PathBuf};

use crate::core::generator::GeneratedOutput;
use crate::error::{KaelError, Result};

/// `.claude/` 디렉토리에 생성된 설정을 기록한다.
pub fn write_output(base: &Path, output: &GeneratedOutput, force: bool) -> Result<Vec<PathBuf>> {
    let claude_dir = base.join(".claude");
    let mut written = Vec::new();

    // CLAUDE.md는 프로젝트 루트에 생성
    write_file(&base.join("CLAUDE.md"), &output.claude_md, force)?;
    written.push(base.join("CLAUDE.md"));

    // settings.json
    std::fs::create_dir_all(&claude_dir)?;
    write_file(
        &claude_dir.join("settings.json"),
        &output.settings_json,
        force,
    )?;
    written.push(claude_dir.join("settings.json"));

    // skills, agents, commands
    for file in &output.skills {
        let path = claude_dir.join(&file.relative_path);
        ensure_parent(&path)?;
        write_file(&path, &file.content, force)?;
        written.push(path);
    }

    for file in &output.agents {
        let path = claude_dir.join(&file.relative_path);
        ensure_parent(&path)?;
        write_file(&path, &file.content, force)?;
        written.push(path);
    }

    for file in &output.commands {
        let path = claude_dir.join(&file.relative_path);
        ensure_parent(&path)?;
        write_file(&path, &file.content, force)?;
        written.push(path);
    }

    Ok(written)
}

/// `.claude/` 디렉토리가 이미 존재하는지 확인한다.
pub fn has_existing_config(base: &Path) -> bool {
    base.join(".claude").exists() || base.join("CLAUDE.md").exists()
}

fn write_file(path: &Path, content: &str, force: bool) -> Result<()> {
    if path.exists() && !force {
        return Err(KaelError::FileExists {
            path: path.to_path_buf(),
        });
    }
    std::fs::write(path, content)?;
    Ok(())
}

fn ensure_parent(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::generator::GeneratedFile;

    fn mock_output() -> GeneratedOutput {
        GeneratedOutput {
            claude_md: "# Test\nGenerated CLAUDE.md".into(),
            settings_json: r#"{"project":{"name":"test"}}"#.into(),
            skills: vec![GeneratedFile {
                relative_path: "skills/rust/error-handling/SKILL.md".into(),
                content: "# Error Handling".into(),
            }],
            agents: vec![GeneratedFile {
                relative_path: "agents/_base/architect.md".into(),
                content: "# Architect".into(),
            }],
            commands: vec![GeneratedFile {
                relative_path: "commands/init.md".into(),
                content: "# Init".into(),
            }],
        }
    }

    #[test]
    fn write_output_creates_files() {
        let dir = tempfile::tempdir().unwrap();
        let output = mock_output();
        let written = write_output(dir.path(), &output, false).unwrap();

        assert!(dir.path().join("CLAUDE.md").exists());
        assert!(dir.path().join(".claude/settings.json").exists());
        assert!(dir
            .path()
            .join(".claude/skills/rust/error-handling/SKILL.md")
            .exists());
        assert!(dir
            .path()
            .join(".claude/agents/_base/architect.md")
            .exists());
        assert!(dir.path().join(".claude/commands/init.md").exists());
        assert_eq!(written.len(), 5);
    }

    #[test]
    fn write_output_refuses_overwrite_without_force() {
        let dir = tempfile::tempdir().unwrap();
        let output = mock_output();
        write_output(dir.path(), &output, false).unwrap();

        let err = write_output(dir.path(), &output, false).unwrap_err();
        assert!(err.to_string().contains("already exists"));
    }

    #[test]
    fn write_output_overwrites_with_force() {
        let dir = tempfile::tempdir().unwrap();
        let output = mock_output();
        write_output(dir.path(), &output, false).unwrap();
        write_output(dir.path(), &output, true).unwrap();

        assert!(dir.path().join("CLAUDE.md").exists());
    }

    #[test]
    fn has_existing_config_detection() {
        let dir = tempfile::tempdir().unwrap();
        assert!(!has_existing_config(dir.path()));

        std::fs::create_dir_all(dir.path().join(".claude")).unwrap();
        assert!(has_existing_config(dir.path()));
    }
}
