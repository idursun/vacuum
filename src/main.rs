mod adapters;
mod application;
mod domain;

use crate::adapters::{DepsUseCase, PomParser};
use crate::adapters::{RestoreUseCase, StoreUseCase};
use crate::application::error::VacuumError;
use crate::application::usecase::UseCase;
use crate::domain::App;
use application::parser::VacuumFileParser;
use std::fs;

fn parse_vacuum_files() -> Result<Vec<App>, VacuumError> {
    let dir = std::fs::read_dir("./apps/")?.filter_map(Result::ok);
    let mut apps = Vec::new();
    for entry in dir {
        if !entry.file_type()?.is_file() {
            continue;
        }

        let content = fs::read_to_string(entry.path()).unwrap();
        let app = PomParser::parse(content)?;
        apps.push(app);
    }
    Ok(apps)
}

fn main() -> Result<(), VacuumError> {
    let mut args = std::env::args();
    if args.len() < 2 {
        println!("Usage: vacuum [command] <folder>");
        println!(" commands:");
        println!(" store   : Store configurations files into folder");
        println!(" restore : Restore configurations files from folder");
        println!(" deps    : List possible dependencies based on configuration files");
        return Ok(());
    }

    let _ = args.nth(0);
    let command = args.nth(0).unwrap_or_else(|| "store".to_owned());
    let output_folder = args.nth(0).unwrap_or_else(|| "output".to_owned());
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
            c @ _ => panic!("unknown command {}", c),
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
