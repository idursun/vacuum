mod adapters;
mod application;
mod domain;

use crate::adapters::{
    parsers::pom_parser::PomParser,
    use_cases::{DepsUseCase, RestoreUseCase, StoreUseCase},
};
use crate::application::error::VacuumError;
use crate::application::parser::VacuumFileParser;
use crate::application::usecase::UseCase;
use crate::domain::App;
use std::fs;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn parse_vacuum_files() -> Result<Vec<App>, VacuumError> {
    let mut apps = Vec::new();
    let dir = std::fs::read_dir("./apps/")?.filter_map(Result::ok);
    for entry in dir {
        if !entry.file_type()?.is_file() {
            continue;
        }

        let content = fs::read_to_string(entry.path())?;
        let app = PomParser::parse(content)?;
        apps.push(app);
    }
    Ok(apps)
}

fn main() -> Result<(), VacuumError> {
    let mut args = std::env::args();
    if args.len() < 2 {
        println!("vacuum {}", VERSION);
        println!("Usage: vacuum [command] <folder>");
        println!(" commands:");
        println!(" store   : Store configurations files into folder");
        println!(" restore : Restore configurations files from folder");
        println!(" deps    : List possible dependencies based on configuration files");
        return Ok(());
    }

    let _ = args.next();
    let command = args.next().unwrap_or_else(|| "store".to_owned());
    let output_folder = args.next().unwrap_or_else(|| "output".to_owned());
    let current_dir = std::env::current_dir()?;

    let apps = parse_vacuum_files()?;
    for app in apps {
        let mut app_dir = current_dir.clone();
        app_dir.push(output_folder.clone());
        app_dir.push(&app.name);

        match command.as_ref() {
            "store" => StoreUseCase::new(app_dir).run(&app)?,
            "restore" => RestoreUseCase::new(app_dir).run(&app)?,
            "deps" => DepsUseCase::new(app_dir).run(&app)?,
            c => panic!("unknown command {}", c),
        };
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn all_vacuum_files_parsed_without_errors() {
        assert!(super::parse_vacuum_files().is_ok());
    }
}
