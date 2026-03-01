use super::repr::*;

use anyhow::{self, bail, ensure, Result};

#[derive(Debug)]
pub struct PathSegment {
    name: String,
    is_task: bool,
}

impl PathSegment {
    pub fn task(name: String) -> Self {
        return Self {
            name,
            is_task: true,
        };
    }

    pub fn project(name: String) -> Self {
        return Self {
            name,
            is_task: false,
        };
    }
    pub fn is_task(&self) -> bool {
        return self.is_task;
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }
}

/**
Path: (project_name/)+(task_name)?
*/
#[derive(Debug)]
pub struct ProjectPath {
    vec: Vec<PathSegment>,
}

impl TryFrom<&str> for ProjectPath {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let value = value.trim();
        ensure!(!value.is_empty(), "path is empty");

        let parts: Vec<&str> = value.split('/').collect();
        ensure!(!parts.is_empty(), "path is empty");

        let ends_with_slash = parts.last().copied() == Some("");
        for (i, p) in parts.iter().enumerate() {
            if p.is_empty() {
                if !(ends_with_slash && i + 1 == parts.len()) {
                    bail!("invalid path: empty segment (leading or consecutive '/')");
                }
            }
        }

        let mut nonempty: Vec<&str> = parts.into_iter().filter(|p| !p.is_empty()).collect();
        ensure!(!nonempty.is_empty(), "invalid path: no segments");

        let (project_parts, task_part) = if ends_with_slash {
            (nonempty.as_slice(), None)
        } else {
            ensure!(
                nonempty.len() >= 2,
                "invalid path: needs at least one project and a task (or end with '/')"
            );
            let task = nonempty.pop().unwrap();
            (nonempty.as_slice(), Some(task))
        };

        ensure!(
            !project_parts.is_empty(),
            "invalid path: must contain at least one project"
        );

        fn validate_name(kind: &str, s: &str) -> Result<()> {
            ensure!(!s.trim().is_empty(), "{kind} name is empty");
            ensure!(
                !s.chars().any(|c| c.is_whitespace() || c == '/'),
                "{kind} name contains invalid characters"
            );
            Ok(())
        }

        let mut vec = Vec::with_capacity(project_parts.len() + task_part.is_some() as usize);

        for p in project_parts {
            validate_name("project", p)?;
            vec.push(PathSegment::project((*p).to_string()));
        }

        if let Some(t) = task_part {
            validate_name("task", t)?;
            vec.push(PathSegment::task(t.to_string()));
        }

        return Ok(ProjectPath { vec });
    }
}

impl ProjectPath {
    pub fn new() -> Self {
        return ProjectPath { vec: Vec::new() };
    }

    pub fn parse<S: AsRef<str>>(s: S) -> Result<Self> {
        return ProjectPath::try_from(s.as_ref());
    }

    pub fn add_task(&mut self, name: String) -> Result<()> {
        if let Some(last) = self.vec.last_mut() {
            if last.is_task() {
                anyhow::bail!("Path already at a task, cannot add a task")
            }
        }

        self.vec.push(PathSegment::task(name));

        Ok(())
    }

    pub fn add_project(&mut self, name: String) -> Result<()> {
        if let Some(last) = self.vec.last_mut() {
            if last.is_task() {
                anyhow::bail!("Path already at a task, cannot add a project")
            }
        }

        self.vec.push(PathSegment::project(name));

        Ok(())
    }
}

trait ProjectStorage {
    fn get_project(&self, path: ProjectPath) -> Result<Project>;
    fn promote_task(&self, path: ProjectPath) -> Result<()>;
    fn get_task(&self, path: ProjectPath) -> Result<Task>;
}
