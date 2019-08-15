use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct App {
    pub name: String,
    pub actions: Vec<Action>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Folder {
    Home,
    Config,
    Custom(String),
    Search(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Action {
    Copy(String),
    CopyGlob(String),
    Context(Folder, Vec<Action>),
    Execute(String),
}
