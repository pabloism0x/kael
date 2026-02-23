use include_dir::{include_dir, Dir};

use crate::error::{KaelError, Result};

// ── Embedded registry ───────────────────────────────────────────────

static SKILLS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/registry/skills");
static AGENTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/registry/agents");
static COMMANDS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/registry/commands");
static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/registry/templates");

// ── Component kind ──────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentKind {
    Skill,
    Agent,
    Command,
}

impl ComponentKind {
    fn dir(&self) -> &'static Dir<'static> {
        match self {
            ComponentKind::Skill => &SKILLS_DIR,
            ComponentKind::Agent => &AGENTS_DIR,
            ComponentKind::Command => &COMMANDS_DIR,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            ComponentKind::Skill => "skill",
            ComponentKind::Agent => "agent",
            ComponentKind::Command => "command",
        }
    }
}

// ── Public API ──────────────────────────────────────────────────────

/// 특정 컴포넌트의 내용을 반환한다.
///
/// - skill: `"rust/async-patterns"` → `registry/skills/rust/async-patterns/SKILL.md`
/// - agent: `"_base/architect"` → `registry/agents/_base/architect.md`
/// - command: `"init"` → `registry/commands/init.md`
pub fn get_component(kind: ComponentKind, name: &str) -> Result<&'static str> {
    let path = match kind {
        ComponentKind::Skill => format!("{name}/SKILL.md"),
        ComponentKind::Agent => format!("{name}.md"),
        ComponentKind::Command => format!("{name}.md"),
    };

    kind.dir()
        .get_file(&path)
        .and_then(|f| f.contents_utf8())
        .ok_or_else(|| KaelError::RegistryNotFound {
            name: format!("{} '{name}'", kind.label()),
        })
}

/// 특정 종류의 모든 컴포넌트 이름 목록을 반환한다.
pub fn list_components(kind: ComponentKind) -> Vec<String> {
    match kind {
        ComponentKind::Skill => list_skills(),
        ComponentKind::Agent => list_agents(),
        ComponentKind::Command => list_commands(),
    }
}

/// 템플릿 파일 내용을 반환한다. (예: `"CLAUDE.md"`, `"settings.json"`)
pub fn get_template(name: &str) -> Result<&'static str> {
    TEMPLATES_DIR
        .get_file(name)
        .and_then(|f| f.contents_utf8())
        .ok_or_else(|| KaelError::RegistryNotFound {
            name: format!("template '{name}'"),
        })
}

/// 컴포넌트 존재 여부를 확인한다.
pub fn has_component(kind: ComponentKind, name: &str) -> bool {
    get_component(kind, name).is_ok()
}

// ── Internal helpers ────────────────────────────────────────────────

/// skills 목록: `"category/skill-name"` 형태
fn list_skills() -> Vec<String> {
    let mut names = Vec::new();
    collect_skills(&SKILLS_DIR, &mut names);
    names.sort();
    names
}

fn collect_skills(dir: &'static Dir<'static>, out: &mut Vec<String>) {
    // 이 디렉토리의 파일 중 SKILL.md가 있으면 스킬로 등록
    let has_skill = dir
        .files()
        .any(|f| f.path().file_name().is_some_and(|name| name == "SKILL.md"));
    if has_skill {
        let path = dir.path().to_string_lossy().into_owned();
        if !path.is_empty() {
            out.push(path);
        }
    }
    // 하위 디렉토리 재귀 탐색
    for sub in dir.dirs() {
        collect_skills(sub, out);
    }
}

/// agents 목록: `"category/agent-name"` 형태
fn list_agents() -> Vec<String> {
    let mut names = Vec::new();
    collect_agents(&AGENTS_DIR, &mut names);
    names.sort();
    names
}

fn collect_agents(dir: &'static Dir<'static>, out: &mut Vec<String>) {
    for file in dir.files() {
        if file.path().extension().is_some_and(|e| e == "md") {
            if let Some(stem) = file.path().file_stem() {
                let parent = file
                    .path()
                    .parent()
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_default();
                if parent.is_empty() {
                    out.push(stem.to_string_lossy().into_owned());
                } else {
                    out.push(format!("{parent}/{}", stem.to_string_lossy()));
                }
            }
        }
    }
    for sub in dir.dirs() {
        collect_agents(sub, out);
    }
}

/// commands 목록: `"command-name"` 형태
fn list_commands() -> Vec<String> {
    let mut names: Vec<String> = COMMANDS_DIR
        .files()
        .filter_map(|f| {
            f.path()
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
        })
        .collect();
    names.sort();
    names
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_skill() {
        let content = get_component(ComponentKind::Skill, "rust/async-patterns").unwrap();
        assert!(content.contains("async"));
    }

    #[test]
    fn get_agent() {
        let content = get_component(ComponentKind::Agent, "_base/architect").unwrap();
        assert!(content.contains("architect") || content.contains("Architect"));
    }

    #[test]
    fn get_command() {
        let content = get_component(ComponentKind::Command, "init").unwrap();
        assert!(content.contains("init") || content.contains("Init"));
    }

    #[test]
    fn get_template() {
        let content = super::get_template("CLAUDE.md").unwrap();
        assert!(content.contains("{{ name }}"));
    }

    #[test]
    fn missing_component_errors() {
        let err = get_component(ComponentKind::Skill, "nonexistent/foo").unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn list_skills_not_empty() {
        let skills = list_components(ComponentKind::Skill);
        assert!(!skills.is_empty());
        assert!(skills.iter().any(|s| s.contains("rust/")));
        assert!(skills.iter().any(|s| s.contains("_common/")));
    }

    #[test]
    fn list_agents_not_empty() {
        let agents = list_components(ComponentKind::Agent);
        assert!(!agents.is_empty());
        assert!(agents.iter().any(|a| a.contains("_base/")));
    }

    #[test]
    fn list_commands_not_empty() {
        let commands = list_components(ComponentKind::Command);
        assert!(!commands.is_empty());
        assert!(commands.contains(&"init".to_string()));
        assert!(commands.contains(&"review".to_string()));
    }

    #[test]
    fn has_component_check() {
        assert!(has_component(ComponentKind::Skill, "rust/async-patterns"));
        assert!(!has_component(ComponentKind::Skill, "nonexistent/foo"));
    }
}
