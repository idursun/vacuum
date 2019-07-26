mod app;
mod executors;

use crate::app::{Action, App, Folder};
use crate::executors::FileSystemExecutor;

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
        //        let serialized = serde_json::to_string(&app).unwrap();
        //        println!("{}", serialized);
        //        println!("{:?}", serde_json::from_str::<App<'_>>(&serialized));

        let result = ron::ser::to_string(app).unwrap();
        println!("{}", result);
        let result = result.as_bytes();
        //println!("{}", result);
        let back = ron::de::from_bytes::<App>(result);
        println!("{:?}", back);

        executors::execute::<FileSystemExecutor>(app);
    }
}
