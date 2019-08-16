mod app;
mod executors;
mod parser;

use crate::app::{Action, App, Folder};
use crate::executors::FileSystemExecutor;
use std::fs;

fn main() {
    let dir = std::fs::read_dir("./apps/").unwrap();
    for entry in dir {
        if let Ok(entry) = entry {
            let content = std::fs::read_to_string(entry.path()).unwrap();
            if let Ok(mut current_dir) = std::env::current_dir() {
                current_dir.push("output");
                if let Ok(app) = parser::app(content) {
                    current_dir.push(&app.name);
                    let executor = FileSystemExecutor::new(current_dir);
                    executors::execute(&executor, &app);
                }
            }
        }
    }
}
