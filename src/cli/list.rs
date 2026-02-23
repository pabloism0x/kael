use console::style;

use crate::core::registry::{self, ComponentKind};
use crate::error::Result;
use crate::ListKind;

pub fn run(kind: ListKind, _installed: bool, _stack: Option<String>) -> Result<()> {
    match kind {
        ListKind::Skills => print_components(ComponentKind::Skill, "Skills"),
        ListKind::Agents => print_components(ComponentKind::Agent, "Agents"),
        ListKind::Commands => print_components(ComponentKind::Command, "Commands"),
        ListKind::All => {
            print_components(ComponentKind::Skill, "Skills");
            println!();
            print_components(ComponentKind::Agent, "Agents");
            println!();
            print_components(ComponentKind::Command, "Commands");
        }
    }
    Ok(())
}

fn print_components(kind: ComponentKind, title: &str) {
    let components = registry::list_components(kind);
    println!("{} ({})", style(title).bold(), components.len());
    for name in &components {
        println!("  {} {}", style("•").dim(), name);
    }
}
