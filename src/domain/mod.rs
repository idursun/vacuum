#[derive(Debug, PartialEq)]
pub struct App {
    pub name: String,
    pub actions: Vec<Action>,
}

#[derive(Debug, PartialEq)]
pub enum Folder {
    Home,
    Config,
    Custom(String),
    Search(String),
}

#[derive(Debug, PartialEq)]
pub enum Action {
    File(String),
    Files(String),
    Context(Folder, Vec<Action>),
    Execute(String, Option<String>),
}
