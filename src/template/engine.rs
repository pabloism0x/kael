use minijinja::{context, Environment, Value};

use crate::core::matcher::MatchResult;
use crate::core::prd::Prd;
use crate::core::registry;
use crate::error::Result;

/// PRD와 매칭 결과로부터 CLAUDE.md 내용을 렌더링한다.
pub fn render_claude_md(prd: &Prd, matched: &MatchResult) -> Result<String> {
    let template_src = registry::get_template("CLAUDE.md")?;
    let ctx = build_context(prd, matched);
    render(template_src, &ctx)
}

/// PRD와 매칭 결과로부터 settings.json 내용을 렌더링한다.
pub fn render_settings_json(prd: &Prd, matched: &MatchResult) -> Result<String> {
    let template_src = registry::get_template("settings.json")?;
    let ctx = build_context(prd, matched);
    render(template_src, &ctx)
}

fn build_context(prd: &Prd, matched: &MatchResult) -> Value {
    context! {
        name => prd.name,
        description => prd.description.as_deref().unwrap_or(""),
        stack => context! {
            language => format!("{:?}", prd.stack.language).to_lowercase(),
            framework => prd.stack.framework,
            database => prd.stack.database,
            infra => prd.stack.infra,
        },
        type => format!("{:?}", prd.project_type).to_lowercase(),
        features => prd.features,
        constraints => prd.constraints,
        agents => matched.agents,
        skills => matched.skills,
        mcp => prd.mcp.as_deref().unwrap_or(&[]),
    }
}

fn render(template_src: &str, ctx: &Value) -> Result<String> {
    let mut env = Environment::new();
    env.add_template("tpl", template_src)?;
    let tpl = env.get_template("tpl")?;
    let rendered = tpl.render(ctx)?;
    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::prd::{Language, ProjectType, Stack};

    fn test_prd() -> Prd {
        Prd {
            name: "my-project".into(),
            description: Some("A test project".into()),
            stack: Stack {
                language: Language::Rust,
                framework: None,
                database: None,
                infra: None,
            },
            project_type: ProjectType::Cli,
            features: Some(vec!["fast-startup".into()]),
            constraints: Some(vec!["no-unsafe".into()]),
            agents: None,
            skills: None,
            mcp: Some(vec!["github".into()]),
            team: None,
        }
    }

    fn test_match() -> MatchResult {
        MatchResult {
            skills: vec!["_common/git-workflow".into(), "rust/async-patterns".into()],
            agents: vec!["_base/architect".into(), "rust/perf-engineer".into()],
            commands: vec!["init".into(), "commit".into()],
        }
    }

    #[test]
    fn render_claude_md_basic() {
        let prd = test_prd();
        let matched = test_match();
        let output = render_claude_md(&prd, &matched).unwrap();

        assert!(output.contains("# my-project"));
        assert!(output.contains("A test project"));
        assert!(output.contains("rust"));
        assert!(output.contains("cargo build"));
        assert!(output.contains("fast-startup"));
        assert!(output.contains("no-unsafe"));
        assert!(output.contains("_base/architect"));
        assert!(output.contains("rust/async-patterns"));
    }

    #[test]
    fn render_settings_json_basic() {
        let prd = test_prd();
        let matched = test_match();
        let output = render_settings_json(&prd, &matched).unwrap();

        assert!(output.contains("my-project"));
        assert!(output.contains("rust"));
        assert!(output.contains("cli"));
        assert!(output.contains("github"));

        // valid JSON check
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert_eq!(parsed["project"]["name"], "my-project");
    }

    #[test]
    fn render_typescript_nextjs() {
        let prd = Prd {
            name: "web-app".into(),
            description: Some("Next.js app".into()),
            stack: Stack {
                language: Language::Typescript,
                framework: Some("nextjs".into()),
                database: Some("postgresql".into()),
                infra: None,
            },
            project_type: ProjectType::Web,
            features: None,
            constraints: None,
            agents: None,
            skills: None,
            mcp: None,
            team: None,
        };
        let matched = MatchResult {
            skills: vec!["typescript/nextjs".into()],
            agents: vec!["_base/architect".into()],
            commands: vec!["init".into()],
        };
        let output = render_claude_md(&prd, &matched).unwrap();

        assert!(output.contains("pnpm"));
        assert!(output.contains("nextjs"));
        assert!(output.contains("postgresql"));
        assert!(output.contains("TypeScript strict mode"));
    }
}
