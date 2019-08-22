mod app;
mod error;
mod executors;
mod parser;
use crate::app::{Action, App, Folder};
use crate::executors::FileSystemExecutor;
use std::fs;

fn main() -> Result<(), error::VacuumError> {
    let dir = std::fs::read_dir("./apps/")?.filter_map(Result::ok);

    let output_folder = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "output".to_owned());
    let current_dir = std::env::current_dir()?;
    for entry in dir {
        if !entry.file_type()?.is_file() {
            continue;
        }

        let content = fs::read_to_string(entry.path()).unwrap();
        let app = parser::parse(content)?;
        let mut app_dir = current_dir.clone();
        app_dir.push(output_folder.clone());
        app_dir.push(&app.name);
        let executor = FileSystemExecutor::new(app_dir, &app.name);
        executors::execute(&executor, &app)?;
    }
    Ok(())
}
