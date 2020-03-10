use crate::adapters::context::deps_context::TargetDirectoryContext;
use crate::application::context::Context;
use crate::application::error::VacuumError;
use crate::application::executor;
use crate::application::usecase::UseCase;
use crate::application::Handler;
use crate::domain::App;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub struct DepsUseCase {
    app_dir: PathBuf,
}

impl DepsUseCase {
    pub fn new(app_dir: PathBuf) -> Self {
        Self { app_dir }
    }
}

struct DependencyAnalyzer {
    analyzer: Box<dyn Analyzer>,
}

struct AlacrittyAnalyzer;
struct VimAnalyzer;
struct NoopAnalyzer;

trait Analyzer {
    fn analyze(&self, file_path: PathBuf);
}

impl Analyzer for AlacrittyAnalyzer {
    fn analyze(&self, file_path: PathBuf) {
        if file_path.file_name().unwrap() == "alacritty.yml" {
            println!("Alacritty is required");
        }
    }
}

impl Analyzer for VimAnalyzer {
    fn analyze(&self, file_path: PathBuf) {
        let mut contents = String::new();
        let mut file = File::open(file_path).unwrap(); //TODO get rid of
        file.read_to_string(&mut contents).unwrap();
        if contents.contains("Plug") {
            println!("VimPlug is required");
        }
    }
}

impl Analyzer for NoopAnalyzer {
    fn analyze(&self, _file_path: PathBuf) {}
}

impl DependencyAnalyzer {
    fn new(app: &App) -> Self {
        let analyzer: Box<dyn Analyzer> = match app.name.as_ref() {
            "alacritty" => Box::new(AlacrittyAnalyzer {}),
            "vim" => Box::new(VimAnalyzer {}),
            _ => Box::new(NoopAnalyzer {}),
        };

        DependencyAnalyzer { analyzer }
    }
}

impl Handler for DependencyAnalyzer {
    type Context = TargetDirectoryContext;

    fn handle_file<S: AsRef<str>>(
        &self,
        ctx: &Self::Context,
        file_name: S,
    ) -> Result<(), VacuumError> {
        let mut file_path = ctx.current();
        file_path.push(file_name.as_ref());
        if file_path.exists() {
            self.analyzer.analyze(file_path.clone());
        }
        Ok(())
    }

    fn handle_files<S: AsRef<str>>(&self, _: &Self::Context, _: S) -> Result<(), VacuumError> {
        Ok(())
    }

    fn handle_execute<S: AsRef<str>>(
        &self,
        _: &Self::Context,
        _: S,
        _: &Option<String>,
    ) -> Result<(), VacuumError> {
        Ok(())
    }
}

impl UseCase for DepsUseCase {
    fn run(&self, app: &App) -> Result<(), VacuumError> {
        let executor = DependencyAnalyzer::new(app);
        executor::execute(
            &executor,
            &TargetDirectoryContext::new(self.app_dir.clone()),
            &app,
        )
    }
}
