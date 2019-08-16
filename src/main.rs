mod app;
mod executors;
mod parser;

use crate::app::{Action, App, Folder};
use crate::executors::FileSystemExecutor;
use std::fs;

fn main() -> std::io::Result<()> {
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
        if let Ok(mut current_dir) = std::env::current_dir() {
            current_dir.push("output");
            match parser::app(content) {
                Ok(app) => {
                    current_dir.push(&app.name);
                    let executor = FileSystemExecutor::new(current_dir);
                    println!("executing {}", app.name);
                    executors::execute(&executor, &app);
                }
                Err(e) => eprintln!(
                    "failed to parse file {}, error: {}",
                    entry.path().display(),
                    e
                ),
            }
        }
    }
    Ok(())
}
