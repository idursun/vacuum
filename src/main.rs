mod app;
mod executors;
mod parser;

use crate::app::{Action, App, Folder};
use crate::executors::FileSystemExecutor;
use std::fs;
fn fish() -> App {
    App {
        name: "fish".into(),
        actions: vec![Action::Context(
            Folder::Config,
            vec![Action::Context(
                Folder::Custom("fish".into()),
                vec![Action::CopyGlob("**/*".into())],
            )],
        )],
    }
}

fn alacritty() -> App {
    App {
        name: "alacritty".into(),
        actions: vec![Action::Context(
            Folder::Config,
            vec![Action::Context(
                Folder::Custom("alacritty".into()),
                vec![Action::Copy("alacritty.yml".into())],
            )],
        )],
    }
}

fn goland() -> App {
    App {
        name: "goland".into(),
        actions: vec![Action::Context(
            Folder::Home,
            vec![Action::Context(
                Folder::Search(".GoLand*".into()),
                vec![Action::Context(
                    Folder::Custom("config".into()),
                    vec![
                        Action::Context(
                            Folder::Custom("keymaps".into()),
                            vec![Action::CopyGlob("*.xml".into())],
                        ),
                        Action::Context(
                            Folder::Custom("options".into()),
                            vec![Action::Copy("editor.xml".into())],
                        ),
                    ],
                )],
            )],
        )],
    }
}

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
