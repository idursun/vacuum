mod app;
mod error;
mod executors;
mod parser;
use crate::app::{Action, App, Folder};
use crate::error::VacuumError;
use crate::executors::{FileSystemExecutor, StoreContext};
use std::fs;

fn parse_vacuum_files() -> Result<Vec<App>, VacuumError> {
    let dir = std::fs::read_dir("./apps/")?.filter_map(Result::ok);
    let mut apps = Vec::new();
    for entry in dir {
        if !entry.file_type()?.is_file() {
            continue;
        }

        let content = fs::read_to_string(entry.path()).unwrap();
        let app = parser::parse(content)?;
        apps.push(app);
    }
    Ok(apps)
}

fn main() -> Result<(), error::VacuumError> {
    let output_folder = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "output".to_owned());
    let current_dir = std::env::current_dir()?;
    let apps = parse_vacuum_files()?;
    for app in apps {
        let mut app_dir = current_dir.clone();
        app_dir.push(output_folder.clone());
        app_dir.push(&app.name);
        let context = StoreContext::new(app_dir);
        let executor = FileSystemExecutor::new(&app.name);
        executors::execute(&executor, &context, &app)?;
    }
    Ok(())
}
