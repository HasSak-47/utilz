use crate::{
    interface::{Path, ProjectStorage},
    repr::Location,
    version::Version,
};

use anyhow::{bail, ensure, Result};
use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

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
    pub fn ensure_project(&self, path: &Path) -> Result<()> {
        ensure!(
            self.project.project.name == path.get_section(0).get_name(),
            "you can only get a name the root project name"
        );

        ensure!(path.len() == 1, "subprojects are not handled yet...");

        return Ok(());
    }
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
        self.ensure_project(&path)?;

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

    fn commit_changes(&mut self) -> Result<()> {
        let path = if let Location::Local(path) = &self.location {
            path
        } else {
            bail!("Cannot edit url data")
        };

        let mut file = File::create(path)?;
        let buf = toml::to_string_pretty(&self.project)?;
        file.write_all(buf.as_bytes())?;

        return Ok(());
    }

    fn create_project(&mut self, path: crate::interface::Path) -> Result<()> {
        bail!("Creating project not available for Basic TOML DB")
    }

    fn insert_task_done(
        &mut self,
        path: crate::interface::Path,
        task: crate::repr::Task,
    ) -> Result<()> {
        self.ensure_project(&path)?;

        self.project.done.insert(
            task.name,
            Task {
                priority: task.priority,
                difficulty: task.difficulty,
            },
        );
        return Ok(());
    }

    fn insert_task_todo(
        &mut self,
        path: crate::interface::Path,
        task: crate::repr::Task,
    ) -> Result<()> {
        self.ensure_project(&path)?;

        self.project.todo.insert(
            task.name,
            Task {
                priority: task.priority,
                difficulty: task.difficulty,
            },
        );
        return Ok(());
    }
    fn mark_done_task(&mut self, path: crate::interface::Path) -> Result<()> {
        self.ensure_project(&path)?;
        let name = path.get_section(1).get_name();
        let task = self.project.todo.remove(&name);
        if let Some(s) = task {
            self.project.done.insert(name, s);
        }

        return Ok(());
    }

    fn mark_todo_task(&mut self, path: crate::interface::Path) -> Result<()> {
        self.ensure_project(&path)?;
        let name = path.get_section(1).get_name();
        let task = self.project.todo.remove(&name);
        if let Some(s) = task {
            self.project.done.insert(name, s);
        }

        return Ok(());
    }
}
