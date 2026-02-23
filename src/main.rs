mod cli;
mod core;
mod error;
mod template;

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "kael",
    version,
    about = "Claude Code configuration framework CLI"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Initialize Claude Code setup for a project
    Init {
        /// Path to PRD.md file
        #[arg(long = "from")]
        from: Option<std::path::PathBuf>,

        /// Minimal setup (CLAUDE.md + commands only)
        #[arg(long)]
        minimal: bool,

        /// Overwrite existing configuration
        #[arg(long)]
        force: bool,
    },

    /// Add a skill, agent, or command
    Add {
        /// Component type (skill, agent, command)
        #[command(subcommand)]
        component: AddComponent,
    },

    /// Remove a skill, agent, or command
    Remove {
        /// Component type (skill, agent, command)
        #[command(subcommand)]
        component: RemoveComponent,
    },

    /// List available or installed components
    List {
        /// Component type (skills, agents, commands, all)
        kind: ListKind,

        /// Show only installed components
        #[arg(long)]
        installed: bool,

        /// Filter by stack
        #[arg(long)]
        stack: Option<String>,
    },

    /// Regenerate CLAUDE.md from PRD
    Generate {
        /// Path to PRD.md file
        #[arg(long = "from")]
        from: Option<std::path::PathBuf>,

        /// Preview without writing files
        #[arg(long)]
        dry_run: bool,
    },

    /// Diagnose current Claude Code configuration
    Doctor,
}

#[derive(clap::Subcommand)]
enum AddComponent {
    /// Add a skill
    Skill { name: String },
    /// Add an agent
    Agent { name: String },
    /// Add a command
    Command { name: String },
}

#[derive(clap::Subcommand)]
enum RemoveComponent {
    /// Remove a skill
    Skill { name: String },
    /// Remove an agent
    Agent { name: String },
    /// Remove a command
    Command { name: String },
}

#[derive(Clone, Debug, clap::ValueEnum)]
enum ListKind {
    Skills,
    Agents,
    Commands,
    All,
}

fn main() -> error::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init {
            from,
            minimal,
            force,
        } => cli::init::run(from, minimal, force),
        Command::Add { component } => match component {
            AddComponent::Skill { name } => cli::add::run_skill(&name),
            AddComponent::Agent { name } => cli::add::run_agent(&name),
            AddComponent::Command { name } => cli::add::run_command(&name),
        },
        Command::Remove { component } => match component {
            RemoveComponent::Skill { name } => cli::remove::run_skill(&name),
            RemoveComponent::Agent { name } => cli::remove::run_agent(&name),
            RemoveComponent::Command { name } => cli::remove::run_command(&name),
        },
        Command::List {
            kind,
            installed,
            stack,
        } => cli::list::run(kind, installed, stack),
        Command::Generate { from, dry_run } => cli::generate::run(from, dry_run),
        Command::Doctor => cli::doctor::run(),
    }
}
