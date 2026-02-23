use std::path::PathBuf;

use console::style;

use crate::core::{generator, prd, project};
use crate::error::{KaelError, Result};

pub fn run(from: Option<PathBuf>, _minimal: bool, force: bool) -> Result<()> {
    let prd_path = resolve_prd_path(from)?;
    let cwd = std::env::current_dir()?;

    println!(
        "{} {}",
        style("→").cyan().bold(),
        style(format!("Parsing {}", prd_path.display())).bold()
    );

    let parsed = prd::parse_prd_file(&prd_path)?;
    let fm = &parsed.frontmatter;

    println!(
        "  {} {} ({:?} / {:?})",
        style("✓").green(),
        fm.name,
        fm.stack.language,
        fm.project_type
    );

    // 기존 설정 감지
    if !force && project::has_existing_config(&cwd) {
        return Err(KaelError::Project {
            message: ".claude/ or CLAUDE.md already exists. Use --force to overwrite.".into(),
        });
    }

    println!(
        "{} {}",
        style("→").cyan().bold(),
        style("Generating configuration").bold()
    );

    let output = generator::generate(fm)?;

    println!(
        "  {} {} skills, {} agents, {} commands",
        style("✓").green(),
        output.skills.len(),
        output.agents.len(),
        output.commands.len()
    );

    println!(
        "{} {}",
        style("→").cyan().bold(),
        style("Writing files").bold()
    );

    let written = project::write_output(&cwd, &output, force)?;

    for path in &written {
        let display = path
            .strip_prefix(&cwd)
            .unwrap_or(path)
            .display()
            .to_string();
        println!("  {} {}", style("+").green(), display);
    }

    println!(
        "\n{} {} files written. Claude Code is ready.",
        style("✓").green().bold(),
        written.len()
    );

    Ok(())
}

fn resolve_prd_path(from: Option<PathBuf>) -> Result<PathBuf> {
    match from {
        Some(path) => {
            if path.exists() {
                Ok(path)
            } else {
                Err(KaelError::Prd {
                    message: format!("File not found: {}", path.display()),
                })
            }
        }
        None => {
            // --from 없으면 현재 디렉토리에서 PRD.md 찾기
            let default = PathBuf::from("PRD.md");
            if default.exists() {
                Ok(default)
            } else {
                Err(KaelError::Prd {
                    message: "No PRD.md found. Use --from <path> to specify.".into(),
                })
            }
        }
    }
}
