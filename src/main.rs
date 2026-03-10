pub mod dbs;
pub mod interface;
pub mod repr;
pub mod version;

use std::{env::current_dir, fs::File, path::PathBuf};

use anyhow::{bail, Result};
use clap::{Args, Parser, Subcommand};

use crate::interface::{Path, ProjectStorage};

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
struct CLI {
    #[command(flatten)]
    opts: Opts,

    #[command(subcommand)]
    command: Commands,
}

#[allow(dead_code)]
fn data_dir() -> PathBuf {
    let mut d = dirs::data_dir().unwrap();
    d.push("project_manager");

    return d;
}

#[derive(Args, Debug, Default, Clone)]
struct Opts {
    #[arg(long)]
    debug: bool,

    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long, default_value_os_t = data_dir())]
    db_path: PathBuf,
}

#[derive(Parser, Clone)]
struct NewProject {
    name: String,

    #[arg(short, long, default_value_os_t = current_dir().unwrap())]
    location: PathBuf,
}

impl NewProject {
    pub fn run(&self, opts: &Opts) -> Result<()> {
        let mut db = crate::dbs::toml::StatusCluster::load(&opts.db_path)?;
        let mut path = Path::new();
        path.add_project(self.name.clone())?;

        let mut project = repr::Project::default();
        project.name = self.name.clone();

        let mut location = self.location.clone();
        location.push("status");
        location.set_extension("toml");

        if std::fs::exists(&location)? {
            bail!("project at {} already exists", location.display());
        }

        db.create_project(path, project, repr::Location::Local(location))?;

        db.save()?;

        return Ok(());
    }
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
    #[arg(short, long)]
    color: bool,
    #[arg(short, long)]
    location: bool,
}

impl List {
    pub fn run(&self, opts: &Opts) -> Result<()> {
        let mut db = crate::dbs::toml::StatusCluster::load(&opts.db_path)?;
        let paths = db.get_projects_path()?;
        println!("{}", paths.len());
        for path in paths {
            println!("{path:?}");
        }

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
    let mut opts = cli.opts;

    if let Err(e) = std::fs::create_dir(&opts.db_path) {
        match e.kind() {
            std::io::ErrorKind::AlreadyExists => {}
            a => bail!("Project Manager Data Error {a:?}"),
        }
    }

    opts.db_path.push("projects");
    opts.db_path.set_extension("toml");
    let _ = File::create(&opts.db_path);

    match cli.command {
        Commands::List(l) => l.run(&opts)?,
        Commands::NewProject(new) => new.run(&opts)?,
        _ => todo!("Todo"),
    }
    return Ok(());
}
