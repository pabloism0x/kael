use crate::core::prd::{Language, Prd, ProjectType};

// ── Matched result ──────────────────────────────────────────────────

/// PRD로부터 자동 매칭된 컴포넌트 목록
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchResult {
    pub skills: Vec<String>,
    pub agents: Vec<String>,
    pub commands: Vec<String>,
}

// ── Public API ──────────────────────────────────────────────────────

/// PRD frontmatter를 분석하여 필요한 skills, agents, commands를 매칭한다.
///
/// 명시적 `agents`/`skills` 필드가 있으면 해당 값을 우선 사용하고,
/// 없으면 `stack.language` + `type`으로 자동 매칭한다.
pub fn match_components(prd: &Prd) -> MatchResult {
    let mut skills = base_skills();
    let mut agents = base_agents();
    let mut commands = base_commands();

    // 명시적 오버라이드 체크
    let has_explicit_skills = prd.skills.as_ref().is_some_and(|s| !s.is_empty());
    let has_explicit_agents = prd.agents.as_ref().is_some_and(|a| !a.is_empty());

    if has_explicit_skills {
        skills = prd.skills.clone().unwrap();
    } else {
        skills.extend(language_skills(
            &prd.stack.language,
            prd.stack.framework.as_deref(),
        ));
        if let Some(infra) = &prd.stack.infra {
            skills.extend(infra_skills(infra));
        }
    }

    if has_explicit_agents {
        agents = prd.agents.clone().unwrap();
    } else {
        agents.extend(language_agents(
            &prd.stack.language,
            prd.stack.framework.as_deref(),
        ));
        agents.extend(type_agents(&prd.project_type));
    }

    commands.extend(type_commands(&prd.project_type));

    dedup(&mut skills);
    dedup(&mut agents);
    dedup(&mut commands);

    MatchResult {
        skills,
        agents,
        commands,
    }
}

// ── Always-included defaults ────────────────────────────────────────

fn base_skills() -> Vec<String> {
    vec!["_common/git-workflow".into(), "_common/ci-cd".into()]
}

fn base_agents() -> Vec<String> {
    vec!["_base/architect".into(), "_base/reviewer".into()]
}

fn base_commands() -> Vec<String> {
    vec!["init".into(), "review".into(), "commit".into()]
}

// ── Language-based matching ─────────────────────────────────────────

fn language_skills(language: &Language, framework: Option<&str>) -> Vec<String> {
    match language {
        Language::Rust => vec![
            "rust/async-patterns".into(),
            "rust/error-handling".into(),
            "rust/memory-optimization".into(),
        ],
        Language::Typescript => {
            let mut s = vec![
                "typescript/react-patterns".into(),
                "typescript/testing".into(),
            ];
            if framework.is_some_and(|f| f.eq_ignore_ascii_case("nextjs")) {
                s.push("typescript/nextjs".into());
            }
            s
        }
        Language::Python => vec!["python/fastapi".into(), "python/ml-ops".into()],
        Language::Go => vec![
            "go/api-patterns".into(),
            "go/concurrency".into(),
            "go/testing".into(),
        ],
    }
}

fn language_agents(language: &Language, framework: Option<&str>) -> Vec<String> {
    match language {
        Language::Rust => vec![
            "rust/perf-engineer".into(),
            "rust/runtime-expert".into(),
            "rust/unsafe-auditor".into(),
        ],
        Language::Typescript => {
            let mut a = vec!["typescript/node-expert".into()];
            if framework.is_some_and(|f| f.eq_ignore_ascii_case("nextjs")) {
                a.push("typescript/fullstack-expert".into());
                a.push("typescript/react-expert".into());
            }
            a
        }
        Language::Python => vec![
            "python/backend-expert".into(),
            "python/ml-engineer".into(),
            "python/data-engineer".into(),
        ],
        Language::Go => vec!["go/systems-expert".into(), "go/api-expert".into()],
    }
}

// ── Type-based matching ─────────────────────────────────────────────

fn type_commands(project_type: &ProjectType) -> Vec<String> {
    match project_type {
        ProjectType::Cli => vec!["test".into(), "release".into()],
        ProjectType::Library => vec!["test".into(), "release".into()],
        ProjectType::Api | ProjectType::Web | ProjectType::Mobile => vec!["test".into()],
    }
}

fn type_agents(project_type: &ProjectType) -> Vec<String> {
    match project_type {
        ProjectType::Cli => vec!["_base/debugger".into()],
        ProjectType::Library => vec!["_base/docs-writer".into()],
        ProjectType::Api => vec!["_base/docs-writer".into(), "_base/test-architect".into()],
        ProjectType::Web => vec!["_base/ui-developer".into()],
        ProjectType::Mobile => vec!["_base/ui-developer".into()],
    }
}

// ── Infra-based matching ────────────────────────────────────────────

fn infra_skills(infra: &[String]) -> Vec<String> {
    let mut skills = Vec::new();
    for item in infra {
        match item.as_str() {
            "docker" => skills.push("infra/docker".into()),
            "kubernetes" => skills.push("infra/kubernetes".into()),
            "github-actions" => skills.push("infra/github-actions".into()),
            _ => {}
        }
    }
    skills
}

// ── Helpers ─────────────────────────────────────────────────────────

/// 순서를 유지하면서 중복을 제거한다.
fn dedup(vec: &mut Vec<String>) {
    let mut seen = std::collections::HashSet::new();
    vec.retain(|item| seen.insert(item.clone()));
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::prd::{Experience, Stack, Team};

    fn make_prd(language: Language, project_type: ProjectType) -> Prd {
        Prd {
            name: "test".into(),
            description: None,
            stack: Stack {
                language,
                framework: None,
                database: None,
                infra: None,
            },
            project_type,
            features: None,
            constraints: None,
            agents: None,
            skills: None,
            mcp: None,
            team: None,
        }
    }

    #[test]
    fn rust_cli_matching() {
        let prd = make_prd(Language::Rust, ProjectType::Cli);
        let result = match_components(&prd);

        // base + rust skills
        assert!(result.skills.contains(&"_common/git-workflow".into()));
        assert!(result.skills.contains(&"_common/ci-cd".into()));
        assert!(result.skills.contains(&"rust/async-patterns".into()));
        assert!(result.skills.contains(&"rust/error-handling".into()));
        assert!(result.skills.contains(&"rust/memory-optimization".into()));

        // base + rust + cli agents
        assert!(result.agents.contains(&"_base/architect".into()));
        assert!(result.agents.contains(&"_base/reviewer".into()));
        assert!(result.agents.contains(&"rust/perf-engineer".into()));
        assert!(result.agents.contains(&"_base/debugger".into()));

        // base + cli commands
        assert!(result.commands.contains(&"init".into()));
        assert!(result.commands.contains(&"review".into()));
        assert!(result.commands.contains(&"commit".into()));
        assert!(result.commands.contains(&"test".into()));
        assert!(result.commands.contains(&"release".into()));
    }

    #[test]
    fn typescript_nextjs_web() {
        let mut prd = make_prd(Language::Typescript, ProjectType::Web);
        prd.stack.framework = Some("nextjs".into());
        let result = match_components(&prd);

        assert!(result.skills.contains(&"typescript/nextjs".into()));
        assert!(result.skills.contains(&"typescript/react-patterns".into()));
        assert!(result
            .agents
            .contains(&"typescript/fullstack-expert".into()));
        assert!(result.agents.contains(&"typescript/react-expert".into()));
        assert!(result.agents.contains(&"_base/ui-developer".into()));
    }

    #[test]
    fn python_api_matching() {
        let prd = make_prd(Language::Python, ProjectType::Api);
        let result = match_components(&prd);

        assert!(result.skills.contains(&"python/fastapi".into()));
        assert!(result.agents.contains(&"python/backend-expert".into()));
        assert!(result.agents.contains(&"_base/docs-writer".into()));
        assert!(result.agents.contains(&"_base/test-architect".into()));
    }

    #[test]
    fn go_library_matching() {
        let prd = make_prd(Language::Go, ProjectType::Library);
        let result = match_components(&prd);

        assert!(result.skills.contains(&"go/api-patterns".into()));
        assert!(result.skills.contains(&"go/concurrency".into()));
        assert!(result.agents.contains(&"go/systems-expert".into()));
        assert!(result.agents.contains(&"_base/docs-writer".into()));
    }

    #[test]
    fn explicit_skills_override() {
        let mut prd = make_prd(Language::Rust, ProjectType::Cli);
        prd.skills = Some(vec!["custom/my-skill".into()]);
        let result = match_components(&prd);

        // 명시적 스킬만 포함, 자동 매칭 스킬은 없어야 함
        assert_eq!(result.skills, vec!["custom/my-skill"]);
        assert!(!result.skills.contains(&"_common/git-workflow".into()));
    }

    #[test]
    fn explicit_agents_override() {
        let mut prd = make_prd(Language::Rust, ProjectType::Cli);
        prd.agents = Some(vec!["custom/my-agent".into()]);
        let result = match_components(&prd);

        // 명시적 에이전트만 포함
        assert_eq!(result.agents, vec!["custom/my-agent"]);
        assert!(!result.agents.contains(&"_base/architect".into()));
    }

    #[test]
    fn infra_skills_added() {
        let mut prd = make_prd(Language::Rust, ProjectType::Cli);
        prd.stack.infra = Some(vec!["docker".into(), "github-actions".into()]);
        let result = match_components(&prd);

        assert!(result.skills.contains(&"infra/docker".into()));
        assert!(result.skills.contains(&"infra/github-actions".into()));
    }

    #[test]
    fn no_duplicates() {
        let prd = make_prd(Language::Rust, ProjectType::Cli);
        let result = match_components(&prd);

        let mut seen = std::collections::HashSet::new();
        for s in &result.skills {
            assert!(seen.insert(s), "duplicate skill: {s}");
        }
        seen.clear();
        for a in &result.agents {
            assert!(seen.insert(a), "duplicate agent: {a}");
        }
        seen.clear();
        for c in &result.commands {
            assert!(seen.insert(c), "duplicate command: {c}");
        }
    }

    #[test]
    fn commands_not_affected_by_explicit_override() {
        let mut prd = make_prd(Language::Rust, ProjectType::Library);
        prd.skills = Some(vec!["custom/skill".into()]);
        prd.agents = Some(vec!["custom/agent".into()]);
        let result = match_components(&prd);

        // commands는 항상 자동 매칭 (명시적 오버라이드 없음)
        assert!(result.commands.contains(&"init".into()));
        assert!(result.commands.contains(&"test".into()));
        assert!(result.commands.contains(&"release".into()));
    }

    #[test]
    fn empty_explicit_lists_trigger_auto_match() {
        let mut prd = make_prd(Language::Rust, ProjectType::Cli);
        prd.skills = Some(vec![]);
        prd.agents = Some(vec![]);
        let result = match_components(&prd);

        // 빈 배열 = 자동 매칭으로 폴백
        assert!(result.skills.contains(&"_common/git-workflow".into()));
        assert!(result.agents.contains(&"_base/architect".into()));
    }

    #[test]
    fn full_prd_matching() {
        let prd = Prd {
            name: "my-project".into(),
            description: Some("A sample project".into()),
            stack: Stack {
                language: Language::Rust,
                framework: Some("custom".into()),
                database: Some("postgresql".into()),
                infra: Some(vec!["docker".into(), "kubernetes".into()]),
            },
            project_type: ProjectType::Cli,
            features: Some(vec!["async-runtime".into()]),
            constraints: Some(vec!["no-tokio".into()]),
            agents: None,
            skills: None,
            mcp: Some(vec!["github".into()]),
            team: Some(Team {
                size: Some(3),
                experience: Some(Experience::Senior),
            }),
        };
        let result = match_components(&prd);

        // language skills + infra skills + base
        assert!(result.skills.contains(&"rust/async-patterns".into()));
        assert!(result.skills.contains(&"infra/docker".into()));
        assert!(result.skills.contains(&"infra/kubernetes".into()));

        // language agents + type agents + base
        assert!(result.agents.contains(&"rust/perf-engineer".into()));
        assert!(result.agents.contains(&"_base/debugger".into()));
    }
}
