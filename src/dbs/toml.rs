use crate::{
    interface::{Path, ProjectStorage},
    repr::Location,
    version::Version,
};

use anyhow::{bail, ensure, Result};
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, fs::File, io::Read};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProjectHeader {
    pub version: Option<Version>,
    pub edition: Version,
    pub name: String,
    pub description: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subprojects: Vec<Path>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Task {
    pub priority: f64,
    pub difficulty: f64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Project {
    pub project: ProjectHeader,

    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub todo: HashMap<String, Task>,

    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub done: HashMap<String, Task>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DB {
    pub project: Project,
    pub location: Location,
}

impl DB {
    pub fn new(location: Location) -> Result<Self> {
        let string = match &location {
            Location::Local(path) => {
                let mut file = File::open(path)?;
                let mut buf = String::new();
                file.read_to_string(&mut buf)?;
                buf
            }
            _ => todo!(),
        };

        let project: Project = toml::from_str(string.as_str())?;

        return Ok(DB {
            project: project,
            location: location,
        });
    }
}

impl ProjectStorage for DB {
    fn promote_task(&self, _: crate::interface::Path) -> anyhow::Result<()> {
        bail!("promoting not available for toml database");
    }
    fn get_project(&self, path: crate::interface::Path) -> Result<crate::repr::Project> {
        ensure!(
            self.project.project.name == path.get_section(0).get_name(),
            "you can only get a name the root project name"
        );

        ensure!(path.len() == 1, "subprojects are not handled yet...");

        return Ok(crate::repr::Project {
            version: self.project.project.version.clone(),
            edition: self.project.project.edition.clone(),

            location: Some(self.location.clone()),
            name: self.project.project.name.clone(),
            description: self.project.project.description.clone(),
            subprojects: Vec::new(),
            todo: self
                .project
                .todo
                .iter()
                .map(|(k, v)| crate::repr::Task {
                    name: k.clone(),
                    priority: v.priority,
                    difficulty: v.difficulty,
                })
                .collect(),
            done: self
                .project
                .done
                .iter()
                .map(|(k, v)| crate::repr::Task {
                    name: k.clone(),
                    priority: v.priority,
                    difficulty: v.difficulty,
                })
                .collect(),
        });
    }

    fn get_task(&self, path: crate::interface::Path) -> Result<crate::repr::Task> {
        ensure!(
            self.project.project.name == path.get_section(0).get_name(),
            "you can only get a name the root project name"
        );

        ensure!(path.len() == 2, "subprojects are not handled yet...");

        ensure!(
            path.get_section(1).is_task(),
            "you can only get a name the root project name"
        );

        let task_name = path.get_section(1).get_name();

        let task = if self.project.todo.contains_key(&task_name) {
            &self.project.todo[&task_name]
        } else if self.project.done.contains_key(&task_name) {
            &self.project.done[&task_name]
        } else {
            bail!("no exisiting task")
        };

        return Ok(crate::repr::Task {
            name: task_name,
            priority: task.priority,
            difficulty: task.difficulty,
        });
    }

    fn commit_changes(&mut self) {
        todo!()
    }

    fn create_project(&mut self, path: crate::interface::Path) -> Result<()> {
        todo!()
    }
    fn create_task(&mut self, path: crate::interface::Path) -> Result<()> {
        todo!()
    }
    fn mark_done_task(&mut self, path: crate::interface::Path) -> Result<()> {
        todo!()
    }
    fn mark_todo_task(&mut self, path: crate::interface::Path) -> Result<()> {
        todo!()
    }
}
