#[derive(Debug, PartialEq)]
pub struct App {
    pub name: String,
    pub actions: Vec<Action>,
    pub dependencies: Option<Vec<Dependency>>,
}

#[derive(Debug, PartialEq)]
pub enum Folder {
    Home,
    Config,
    Local,
    Custom(String),
    Search(String),
}

#[derive(Debug, PartialEq)]
pub enum DependencyCheck {
    Contains(String, String),
    Exists(String),
}

#[derive(Debug, PartialEq)]
pub enum Action {
    File(String, Option<Vec<DependencyCheck>>),
    Files(String),
    Context(Folder, Vec<Action>),
    Execute(String, Option<String>),
}

#[derive(Debug, PartialEq)]
pub struct Dependency {
    pub name: String,
    pub block: String,
}
