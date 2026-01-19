#![allow(unused_imports)]
use std::fs::File;
use std::io::{Read, Write};

use project_manager_api as pm_api;

use ly::log::{self, write::ANSI};
use pm_api::project::Project;
use pm_api::task::Task;
use pm_api::desc::Descriptor;
use pm_api::Database;
use serde;
use anyhow::{anyhow, Result};

const TEST_PROJECT: &str = include_str!("./test_project.json");
const TEST_TASK: &str = include_str!("./test_task.json");
use ly::log::prelude::*;

struct ReaderWriter{ }

#[test]
fn test_add_project() -> Result<()>{
    let ansi = ANSI::new();
    log::set_logger(ansi);
    log::set_level(log::Level::Log);

    let tree : Project = serde_json::from_str(TEST_PROJECT)?;
    let mut pool = Database::default();
    pool.add_full_project(tree)?;

    println!("{pool:?}");
    Ok(())
}

#[test]
fn test_add_task() -> Result<()>{
    let ansi = ANSI::new();
    log::set_logger(ansi);
    log::set_level(log::Level::Log);

    let project : Project = serde_json::from_str(TEST_PROJECT)?;
    let mut pool = Database::default();

    pool.add_full_project(project)?;

    let task: Task = serde_json::from_str(TEST_TASK)?;
    println!("{task:?}");
    pool.add_full_task(task)?;

    println!("{pool:?}");
    Ok(())
}

#[test]
fn unfold() -> Result<()>{
    let ansi = ANSI::new();

    let project : Project = serde_json::from_str(TEST_PROJECT)?;
    let mut pool = Database::default();

    pool.add_full_project(project)?;

    let task: Task = serde_json::from_str(TEST_TASK)?;
    pool.add_full_task(task)?;

    println!("here!");

    let _projects = pool.build_project_trees()?;

    println!("trees!");

    let _task = pool.build_task_tree(0)?;

    println!("finished!");

    Ok(())
}


fn proc_task(depth: usize, max: usize) -> Task{
    let mut task = Task::new().desc(Descriptor::new());
    let child = rand10() + depth < max;
    if child && depth < max{
        let child_count : usize = rand10();
        for _ in 0..child_count{
            task.childs.push(proc_task(depth + 1, max));
        }
    }

    return task;
}

fn proc_project(depth: usize, max: usize) -> Project{
    let mut project = Project::new().desc(
        Descriptor::new()
    );
    let child = rand10() + depth < max;
    if child && depth < max{
        let child_count : usize = rand10();
        for _ in 0..child_count{
            project.childs.push(proc_project(depth + 1, max));
        }
    }
    let task = rand10() + depth < 2 * max;
    if task && depth < max{
        let task_count : usize = rand10();
        for _ in 0..task_count{
            project.tasks.push(proc_task(depth + 1, max));
        }
    }

    return project;
}

#[test]
fn proc_test() -> Result<()>{
    let project = proc_project(0, 4);
    let mut pool = Database::default();

    pool.add_full_project(project.clone())?;
    let mut tree = pool.build_project_trees()?;
    if tree.len() != 1{
        return Err(anyhow!("project not returned"));
    }
    let returned_project = tree.pop().ok_or(anyhow!("how???"))?;
    if project != returned_project {
        println!("projects are different")
    }


    Ok(())
}

#[test]
#[ignore = "it is still weak"]
fn stress_test() -> Result<()>{
    use rand::prelude::*;
    let n_rp = u16::MAX as usize;
    let mut root_projects = Vec::with_capacity(n_rp);
    for i in 0..n_rp{
        println!("creating project: {i}");
        let project = proc_project(0, u8::MAX as usize);
        root_projects.push(project);
    }

    Ok(())
}

fn rand10() -> usize{
    use rand::random;
    random::<usize>() % 10
}
