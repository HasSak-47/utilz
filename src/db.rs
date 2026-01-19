use anyhow::Result;
use std::path::{Path, PathBuf};

// database entries
struct Project {
    id: usize,
    name: String,
    subproject: Vec<Project>,
}

struct Task {
    id: usize,
    name: String,
    project: usize,
}

struct ExternalDatabase {
    id: usize,
    con: StoreConnection,
}

enum DatabaseKind {
    SQLite,
    Toml,
}

enum StoreConnection {
    Path(PathBuf),
    Url(String),
}

trait LocalStorage {
    fn get_project(&self, path: usize) -> Result<Project>;
    fn get_task(&self, id: usize) -> Result<Task>;
}

struct SQLITEDatabase {}

impl SQLITEDatabase {
    pub fn init<P: AsRef<Path>>(path: P) -> Self {
        todo!()
    }
}

impl LocalStorage for SQLITEDatabase {
    fn get_project(&self, path: usize) -> Result<Project> {
        todo!()
    }

    fn get_task(&self, id: usize) -> Result<Task> {
        todo!()
    }
}
