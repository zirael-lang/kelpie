use crate::ctx::KelpieContext;
use crate::{Package, PackageId, Project, ProjectId, ProjectKind};
use anyhow::Result;
use petgraph::{
    Direction,
    algo::toposort,
    graph::{DiGraph, NodeIndex},
};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct DependencyGraph {
    graph: DiGraph<PackageId, ()>,
    package_to_node: HashMap<PackageId, NodeIndex>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            package_to_node: HashMap::new(),
        }
    }

    fn add_package(&mut self, package_id: PackageId) -> NodeIndex {
        *self
            .package_to_node
            .entry(package_id)
            .or_insert_with(|| self.graph.add_node(package_id))
    }

    fn add_dependency(&mut self, dependent: PackageId, dependency: PackageId) {
        let dependent_node = self.add_package(dependent);
        let dependency_node = self.add_package(dependency);

        if !self.graph.contains_edge(dependent_node, dependency_node) {
            self.graph.add_edge(dependent_node, dependency_node, ());
        }
    }

    pub fn build_from_project(&mut self, ctx: &KelpieContext, project_id: ProjectId) -> Result<()> {
        if let Some(project) = ctx.get_project(project_id) {
            match &project.kind {
                ProjectKind::Package(package_id) => {
                    for dep in &project.dependencies {
                        self.add_dependency(*package_id, dep.id);
                    }
                }
                ProjectKind::Workspace(workspace) => {
                    for &member_id in &workspace.members {
                        for dep in &project.dependencies {
                            self.add_dependency(member_id, dep.id);
                        }

                        if let Some(member_project_id) = ctx.find_project_by_package_id(member_id) {
                            if let Some(member_project) = ctx.get_project(member_project_id) {
                                for dep in &member_project.dependencies {
                                    self.add_dependency(member_id, dep.id);
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn get_compilation_order(&self) -> Result<Vec<PackageId>> {
        match toposort(&self.graph, None) {
            Ok(mut order) => {
                order.reverse();
                Ok(order.into_iter().map(|node| self.graph[node]).collect())
            }
            Err(_) => anyhow::bail!("Circular dependency detected in the project"),
        }
    }
}
