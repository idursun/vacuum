use serde::{Deserialize, Serialize};

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
