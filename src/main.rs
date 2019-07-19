use crate::executors::FileSystemExecutor;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

mod executors;

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    pub name: String,
    pub actions: Vec<Action>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Folder {
    Home,
    Config,
    Custom(String),
    Search(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Copy(String),
    CopyGlob(String),
    Context(Folder, Vec<Action>),
    Execute(String),
}

fn fish() -> App {
    App {
        name: "fish".into(),
        actions: vec![Action::Context(
            Folder::Config,
            vec![Action::CopyGlob("**/*".into())],
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
    }
}

fn main() {
    let apps = vec![fish(), webstorm()];
    for app in &apps {
        //        let serialized = serde_json::to_string(&app).unwrap();
        //        println!("{}", serialized);
        //        println!("{:?}", serde_json::from_str::<App<'_>>(&serialized));

        let pretty = PrettyConfig::default();
        let result = ron::ser::to_string_pretty(app, pretty).unwrap();
        println!("{}", result);
        let result = result.as_bytes();
        //println!("{}", result);
        let back = ron::de::from_bytes::<App>(result);
        println!("{:?}", back);

        executors::execute::<FileSystemExecutor>(app);
    }
}
