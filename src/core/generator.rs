use crate::core::matcher::{self, MatchResult};
use crate::core::prd::Prd;
use crate::core::registry::{self, ComponentKind};
use crate::error::Result;
use crate::template::engine;

/// 생성된 모든 파일 내용을 담는 구조체
#[derive(Debug)]
pub struct GeneratedOutput {
    pub claude_md: String,
    pub settings_json: String,
    pub skills: Vec<GeneratedFile>,
    pub agents: Vec<GeneratedFile>,
    pub commands: Vec<GeneratedFile>,
}

#[derive(Debug)]
pub struct GeneratedFile {
    /// `.claude/` 기준 상대 경로 (예: `skills/rust/async-patterns/SKILL.md`)
    pub relative_path: String,
    pub content: String,
}

/// PRD frontmatter로부터 전체 `.claude/` 구성을 생성한다.
pub fn generate(prd: &Prd) -> Result<GeneratedOutput> {
    let matched = matcher::match_components(prd);
    generate_with_match(prd, &matched)
}

/// PRD와 명시적 MatchResult로부터 생성한다. (테스트용)
pub fn generate_with_match(prd: &Prd, matched: &MatchResult) -> Result<GeneratedOutput> {
    let claude_md = engine::render_claude_md(prd, matched)?;
    let settings_json = engine::render_settings_json(prd, matched)?;

    let skills = matched
        .skills
        .iter()
        .filter_map(|name| {
            registry::get_component(ComponentKind::Skill, name)
                .ok()
                .map(|content| GeneratedFile {
                    relative_path: format!("skills/{name}/SKILL.md"),
                    content: content.to_string(),
                })
        })
        .collect();

    let agents = matched
        .agents
        .iter()
        .filter_map(|name| {
            registry::get_component(ComponentKind::Agent, name)
                .ok()
                .map(|content| GeneratedFile {
                    relative_path: format!("agents/{name}.md"),
                    content: content.to_string(),
                })
        })
        .collect();

    let commands = matched
        .commands
        .iter()
        .filter_map(|name| {
            registry::get_component(ComponentKind::Command, name)
                .ok()
                .map(|content| GeneratedFile {
                    relative_path: format!("commands/{name}.md"),
                    content: content.to_string(),
                })
        })
        .collect();

    Ok(GeneratedOutput {
        claude_md,
        settings_json,
        skills,
        agents,
        commands,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::prd::{Language, ProjectType, Stack};

    #[test]
    fn generate_rust_cli() {
        let prd = Prd {
            name: "my-cli".into(),
            description: Some("A Rust CLI tool".into()),
            stack: Stack {
                language: Language::Rust,
                framework: None,
                database: None,
                infra: None,
            },
            project_type: ProjectType::Cli,
            features: None,
            constraints: None,
            agents: None,
            skills: None,
            mcp: None,
            team: None,
        };

        let output = generate(&prd).unwrap();

        assert!(output.claude_md.contains("my-cli"));
        assert!(output.claude_md.contains("cargo build"));
        assert!(!output.skills.is_empty());
        assert!(!output.agents.is_empty());
        assert!(!output.commands.is_empty());

        // skills에 올바른 경로 형식
        assert!(output
            .skills
            .iter()
            .any(|s| s.relative_path.starts_with("skills/")));
    }

    #[test]
    fn generate_settings_json_valid() {
        let prd = Prd {
            name: "test".into(),
            description: None,
            stack: Stack {
                language: Language::Python,
                framework: Some("fastapi".into()),
                database: None,
                infra: None,
            },
            project_type: ProjectType::Api,
            features: None,
            constraints: None,
            agents: None,
            skills: None,
            mcp: Some(vec!["github".into()]),
            team: None,
        };

        let output = generate(&prd).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&output.settings_json).unwrap();
        assert_eq!(parsed["project"]["name"], "test");
        assert_eq!(parsed["project"]["language"], "python");
    }
}
