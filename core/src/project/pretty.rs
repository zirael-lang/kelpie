use crate::{KelpieContext, ProjectId, ProjectKind};

pub fn print_project_tree(ctx: &KelpieContext, project_id: ProjectId, indent: usize) {
    if let Some(project) = ctx.get_project(project_id) {
        let indent_str = "  ".repeat(indent);

        match &project.kind {
            ProjectKind::Workspace(workspace) => {
                println!(
                    "{}📁 Workspace with {} members",
                    indent_str,
                    workspace.members.len()
                );

                for &member_id in &workspace.members {
                    if let Some(package) = ctx.get_package(member_id) {
                        println!("{}  📦 {}", indent_str, package.name);
                    }
                }
            }
            ProjectKind::Package(package) => {
                let package = ctx.get_package(*package).unwrap();
                println!(
                    "{}📦 Package: {} v{}",
                    indent_str, package.name, package.version
                );
            }
        }

        if !project.dependencies.is_empty() {
            println!("{}  Dependencies:", indent_str);
            for dep in &project.dependencies {
                if let Some(dep_package) = ctx.get_package(dep.id) {
                    println!("{}    - {} v{}", indent_str, dep_package.name, dep.version);
                }
            }
        }
    }
}
