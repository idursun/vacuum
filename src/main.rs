mod app;
mod executors;

use crate::app::{Action, App, Folder};
use crate::executors::file_system_executor;

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

fn webstorm() -> App {
    App {
        name: "webstorm".into(),
        actions: vec![Action::Context(
            Folder::Home,
            vec![Action::Context(
                Folder::Search(".WebStorm*".into()),
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
    let apps = vec![fish(), alacritty(), webstorm(), goland()];
    for app in &apps {
        let pretty = ron::ser::PrettyConfig::default();
        let result = ron::ser::to_string_pretty(app, pretty).unwrap();
        println!("{}", result);
        let back = ron::de::from_bytes::<App>(result.as_bytes());
        println!("{:?}", back);
        if let Ok(mut current_dir) = std::env::current_dir() {
            current_dir.push("output");
            current_dir.push(&app.name);
            let executor = file_system_executor(current_dir);
            executors::execute(&executor, app);
        }
    }
}
