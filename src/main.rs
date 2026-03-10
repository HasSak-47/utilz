pub mod dbs;
pub mod interface;
pub mod repr;
pub mod version;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

use crate::interface::ProjectStorage;

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
struct CLI {
    #[command(flatten)]
    opts: Opts,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Args, Debug, Default, Clone)]
struct Opts {
    #[arg(long, global = true)]
    debug: bool,

    #[arg(short, long, global = true)]
    verbose: bool,

    #[arg(short, long, global = true)]
    db_path: PathBuf,
}

#[derive(Parser, Clone)]
struct NewProject {
    name: String,
    location: PathBuf,
}

#[derive(Parser, Clone)]
struct SetSubproject {
    parent: String,
    child: String,
}

#[derive(Parser, Clone)]
struct InitProject {
    name: String,
    location: PathBuf,
}

#[derive(Parser, Clone)]
struct DeleteProject {
    name: String,
}

#[derive(Parser, Clone)]
struct AddTask {
    project: String,
    name: String,
    todo: bool,
    difficulty: f64,
    priority: f64,
}

#[derive(Parser, Clone)]
struct RemoveTask {
    name: String,
}

#[derive(Parser, Clone)]
struct PromoteTask {
    name: String,
    new_path: PathBuf,
}

#[derive(Parser, Clone)]
struct MarkTask {
    name: String,
    todo: bool,
}

#[derive(Parser, Clone)]
struct List {
    color: bool,
    location: bool,
}

impl List {
    pub fn run(&self, opts: &Opts) -> Result<()> {
        let mut db = crate::dbs::toml::StatusCluster::load(&opts.db_path)?;
        let paths = db.get_projects_path()?;

        return Ok(());
    }
}

#[derive(Subcommand, Clone)]
enum Commands {
    NewProject(NewProject),
    SetSubproject(SetSubproject),
    InitProject(InitProject),
    DeleteProject(DeleteProject),
    AddTask(AddTask),
    RemoveTask(RemoveTask),
    PromoteTask(PromoteTask),
    MarkTask(MarkTask),
    List(List),
}

fn main() -> Result<()> {
    let cli = CLI::parse();
    return Ok(());
}
