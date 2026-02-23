use std::path::Path;

use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};
use serde::{Deserialize, Serialize};

use crate::error::{KaelError, Result};

// ── Data types ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prd {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub stack: Stack,
    #[serde(rename = "type")]
    pub project_type: ProjectType,
    #[serde(default)]
    pub features: Option<Vec<String>>,
    #[serde(default)]
    pub constraints: Option<Vec<String>>,
    #[serde(default)]
    pub agents: Option<Vec<String>>,
    #[serde(default)]
    pub skills: Option<Vec<String>>,
    #[serde(default)]
    pub mcp: Option<Vec<String>>,
    #[serde(default)]
    pub team: Option<Team>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stack {
    pub language: Language,
    #[serde(default)]
    pub framework: Option<String>,
    #[serde(default)]
    pub database: Option<String>,
    #[serde(default)]
    pub infra: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Rust,
    Typescript,
    Python,
    Go,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Library,
    Cli,
    Web,
    Api,
    Mobile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    #[serde(default)]
    pub size: Option<u32>,
    #[serde(default)]
    pub experience: Option<Experience>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Experience {
    Junior,
    Mid,
    Senior,
}

// ── Parsed PRD (frontmatter + body) ─────────────────────────────────

#[derive(Debug, Clone)]
pub struct ParsedPrd {
    pub frontmatter: Prd,
    pub body: PrdBody,
}

#[derive(Debug, Clone, Default)]
pub struct PrdBody {
    pub sections: Vec<Section>,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub heading: String,
    pub level: u32,
    pub content: String,
}

// ── Public API ──────────────────────────────────────────────────────

/// PRD.md 파일을 읽고 파싱하여 `ParsedPrd`를 반환한다.
pub fn parse_prd_file(path: &Path) -> Result<ParsedPrd> {
    let content = std::fs::read_to_string(path)?;
    parse_prd(&content)
}

/// PRD 문자열을 파싱하여 `ParsedPrd`를 반환한다.
pub fn parse_prd(content: &str) -> Result<ParsedPrd> {
    let (yaml_str, body_str) = extract_frontmatter(content)?;
    let frontmatter = parse_frontmatter(yaml_str)?;
    let body = parse_body(body_str);
    Ok(ParsedPrd { frontmatter, body })
}

// ── Internal helpers ────────────────────────────────────────────────

/// `---` 구분자로 YAML frontmatter와 markdown body를 분리한다.
fn extract_frontmatter(content: &str) -> Result<(&str, &str)> {
    let trimmed = content.trim_start();

    if !trimmed.starts_with("---") {
        return Err(KaelError::Prd {
            message: "PRD must start with YAML frontmatter (---)".into(),
        });
    }

    // 첫 번째 "---" 이후에서 두 번째 "---"를 찾는다
    let after_first = &trimmed[3..];
    let closing = after_first.find("\n---").ok_or(KaelError::Prd {
        message: "Missing closing frontmatter delimiter (---)".into(),
    })?;

    let yaml_str = &after_first[..closing];
    let rest = &after_first[closing + 4..]; // skip "\n---"

    // body는 closing --- 뒤의 줄바꿈 이후부터
    let body_str = rest.strip_prefix('\n').unwrap_or(rest);

    Ok((yaml_str, body_str))
}

/// YAML frontmatter 문자열을 `Prd` 구조체로 역직렬화한다.
fn parse_frontmatter(yaml_str: &str) -> Result<Prd> {
    let prd: Prd = serde_yaml_ng::from_str(yaml_str)?;
    Ok(prd)
}

/// Markdown body를 헤딩별 섹션으로 파싱한다.
fn parse_body(markdown: &str) -> PrdBody {
    let parser = Parser::new(markdown);
    let mut sections = Vec::new();
    let mut current_heading: Option<(String, u32)> = None;
    let mut current_content = String::new();
    let mut in_heading = false;
    let mut heading_text = String::new();
    let mut heading_level = 0u32;

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                // 이전 섹션을 저장
                if let Some((heading, level)) = current_heading.take() {
                    sections.push(Section {
                        heading,
                        level,
                        content: current_content.trim().to_string(),
                    });
                    current_content.clear();
                }
                in_heading = true;
                heading_text.clear();
                heading_level = heading_level_to_u32(level);
            }
            Event::End(TagEnd::Heading(_)) => {
                in_heading = false;
                current_heading = Some((heading_text.clone(), heading_level));
            }
            Event::Text(text) | Event::Code(text) => {
                if in_heading {
                    heading_text.push_str(&text);
                } else if current_heading.is_some() {
                    current_content.push_str(&text);
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                if !in_heading && current_heading.is_some() {
                    current_content.push('\n');
                }
            }
            _ => {}
        }
    }

    // 마지막 섹션 저장
    if let Some((heading, level)) = current_heading {
        sections.push(Section {
            heading,
            level,
            content: current_content.trim().to_string(),
        });
    }

    PrdBody { sections }
}

fn heading_level_to_u32(level: HeadingLevel) -> u32 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const FULL_PRD: &str = r#"---
name: "my-project"
description: "A sample project"
stack:
  language: rust
  framework: custom
  database: postgresql
  infra:
    - docker
    - github-actions
type: cli
features:
  - async-runtime
  - zero-copy-patterns
constraints:
  - no-tokio-dependency
  - benchmark-before-optimize
agents:
  - _base/architect
  - rust/perf-engineer
skills:
  - _common/git-workflow
  - rust/async-patterns
mcp:
  - github
team:
  size: 3
  experience: senior
---

# My Project

This is the project description.

## Architecture

The system uses a layered architecture.

## Goals

- Fast startup
- Low memory usage
"#;

    const MINIMAL_PRD: &str = r#"---
name: "minimal"
stack:
  language: python
type: api
---
"#;

    #[test]
    fn parse_full_prd() {
        let parsed = parse_prd(FULL_PRD).unwrap();
        let fm = &parsed.frontmatter;

        assert_eq!(fm.name, "my-project");
        assert_eq!(fm.description.as_deref(), Some("A sample project"));
        assert_eq!(fm.stack.language, Language::Rust);
        assert_eq!(fm.stack.framework.as_deref(), Some("custom"));
        assert_eq!(fm.stack.database.as_deref(), Some("postgresql"));
        assert_eq!(
            fm.stack.infra.as_deref(),
            Some(["docker", "github-actions"].map(String::from).as_slice())
        );
        assert_eq!(fm.project_type, ProjectType::Cli);
        assert_eq!(fm.features.as_ref().unwrap().len(), 2);
        assert_eq!(fm.constraints.as_ref().unwrap().len(), 2);
        assert_eq!(fm.agents.as_ref().unwrap().len(), 2);
        assert_eq!(fm.skills.as_ref().unwrap().len(), 2);
        assert_eq!(fm.mcp.as_ref().unwrap(), &["github"]);
        let team = fm.team.as_ref().unwrap();
        assert_eq!(team.size, Some(3));
        assert_eq!(team.experience, Some(Experience::Senior));
    }

    #[test]
    fn parse_minimal_prd() {
        let parsed = parse_prd(MINIMAL_PRD).unwrap();
        let fm = &parsed.frontmatter;

        assert_eq!(fm.name, "minimal");
        assert!(fm.description.is_none());
        assert_eq!(fm.stack.language, Language::Python);
        assert!(fm.stack.framework.is_none());
        assert_eq!(fm.project_type, ProjectType::Api);
        assert!(fm.features.is_none());
        assert!(fm.constraints.is_none());
        assert!(fm.agents.is_none());
        assert!(fm.skills.is_none());
        assert!(fm.mcp.is_none());
        assert!(fm.team.is_none());
    }

    #[test]
    fn parse_body_sections() {
        let parsed = parse_prd(FULL_PRD).unwrap();
        let sections = &parsed.body.sections;

        assert_eq!(sections.len(), 3);
        assert_eq!(sections[0].heading, "My Project");
        assert_eq!(sections[0].level, 1);
        assert_eq!(sections[1].heading, "Architecture");
        assert_eq!(sections[1].level, 2);
        assert_eq!(sections[2].heading, "Goals");
        assert_eq!(sections[2].level, 2);
    }

    #[test]
    fn missing_name_errors() {
        let prd = r#"---
stack:
  language: rust
type: cli
---
"#;
        let err = parse_prd(prd).unwrap_err();
        assert!(err.to_string().contains("missing field"));
    }

    #[test]
    fn missing_stack_errors() {
        let prd = r#"---
name: "test"
type: cli
---
"#;
        let err = parse_prd(prd).unwrap_err();
        assert!(err.to_string().contains("missing field"));
    }

    #[test]
    fn no_frontmatter_errors() {
        let err = parse_prd("# Just a markdown file").unwrap_err();
        assert!(err.to_string().contains("frontmatter"));
    }

    #[test]
    fn unclosed_frontmatter_errors() {
        let prd = "---\nname: test\n";
        let err = parse_prd(prd).unwrap_err();
        assert!(err.to_string().contains("closing"));
    }

    #[test]
    fn frontmatter_only_no_body() {
        let parsed = parse_prd(MINIMAL_PRD).unwrap();
        assert!(parsed.body.sections.is_empty());
    }

    #[test]
    fn invalid_language_errors() {
        let prd = r#"---
name: "test"
stack:
  language: java
type: cli
---
"#;
        let err = parse_prd(prd).unwrap_err();
        assert!(err.to_string().contains("unknown variant"));
    }

    #[test]
    fn invalid_type_errors() {
        let prd = r#"---
name: "test"
stack:
  language: rust
type: desktop
---
"#;
        let err = parse_prd(prd).unwrap_err();
        assert!(err.to_string().contains("unknown variant"));
    }
}
