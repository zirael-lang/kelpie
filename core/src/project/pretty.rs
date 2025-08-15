use crate::{KelpieContext, ProjectId, ProjectKind};
use std::io::{self, Write};

const TREE_BRANCH: &str = "├── ";
const TREE_CORNER: &str = "└── ";
const TREE_VERTICAL: &str = "│   ";
const TREE_SPACE: &str = "    ";

pub fn print_project_tree(ctx: &KelpieContext, project_id: ProjectId, indent: usize) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    print_project_node(&mut stdout, ctx, project_id, &vec![false; indent])
}

fn print_project_node(
    w: &mut impl Write,
    ctx: &KelpieContext,
    project_id: ProjectId,
    parents: &[bool],
) -> io::Result<()> {
    if let Some(project) = ctx.get_project(project_id) {
        for &has_sibling in &parents[..parents.len().saturating_sub(1)] {
            write!(w, "{}", if has_sibling { TREE_VERTICAL } else { TREE_SPACE })?;
        }
        if !parents.is_empty() {
            write!(
                w,
                "{}",
                if *parents.last().unwrap() { TREE_BRANCH } else { TREE_CORNER }
            )?;
        }

        match &project.kind {
            ProjectKind::Workspace(workspace) => {
                writeln!(
                    w,
                    "Workspace ({} members)",
                    workspace.members.len()
                )?;

                let member_count = workspace.members.len();
                for (idx, &member_id) in workspace.members.iter().enumerate() {
                    if let Some(package) = ctx.get_package(member_id) {
                        let mut new_parents = parents.to_vec();
                        new_parents.push(idx < member_count - 1);

                        for &has_sibling in parents {
                            write!(w, "{}", if has_sibling { TREE_VERTICAL } else { TREE_SPACE })?;
                        }
                        write!(
                            w,
                            "{}{}",
                            if idx < member_count - 1 { TREE_BRANCH } else { TREE_CORNER },
                            package.name
                        )?;
                        writeln!(w, " v{}", package.version)?;
                    }
                }
            }
            ProjectKind::Package(package) => {
                if let Some(package) = ctx.get_package(*package) {
                    writeln!(w, "{} v{}", package.name, package.version)?;
                }
            }
        }

        if !project.dependencies.is_empty() {
            for &has_sibling in parents {
                write!(w, "{}", if has_sibling { TREE_VERTICAL } else { TREE_SPACE })?;
            }
            writeln!(w, "{}Dependencies:", TREE_BRANCH)?;

            let dep_count = project.dependencies.len();
            for (idx, dep) in project.dependencies.iter().enumerate() {
                if let Some(dep_package) = ctx.get_package(dep.id) {
                    for &has_sibling in parents {
                        write!(w, "{}", if has_sibling { TREE_VERTICAL } else { TREE_SPACE })?;
                    }
                    write!(
                        w,
                        "{}{}",
                        TREE_VERTICAL,
                        if idx < dep_count - 1 { TREE_BRANCH } else { TREE_CORNER }
                    )?;
                    writeln!(w, "{} v{}", dep_package.name, dep.version)?;
                }
            }
        }
    }
    Ok(())
}