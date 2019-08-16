mod app;
mod error;
mod executors;
mod parser;

use crate::app::{Action, App, Folder};
use crate::executors::FileSystemExecutor;
use std::fs;

fn main() -> Result<(), error::VacuumError> {
    let dir = std::fs::read_dir("./apps/")?
        .filter_map(Result::ok)
        .into_iter();

    for entry in dir {
        if let Ok(ft) = entry.file_type() {
            if !ft.is_file() {
                continue;
            }
        }

        let content = fs::read_to_string(entry.path()).unwrap();
        let app = parser::app(content)?;
        let mut current_dir = std::env::current_dir()?;
        current_dir.push("output");
        current_dir.push(&app.name);
        let executor = FileSystemExecutor::new(current_dir);
        println!("executing {}", app.name);
        executors::execute(&executor, &app);
    }
    Ok(())
}
