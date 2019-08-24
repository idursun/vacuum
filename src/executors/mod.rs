mod context;
mod file_system_executor;
mod ops;

use crate::error::VacuumError;
use crate::{Action, Folder};
pub use context::Context;
pub use file_system_executor::FileSystemExecutor;
pub use ops::Ops;

fn execute_actions<E>(executor: &E, actions: &[Action]) -> Result<(), VacuumError>
where
    E: Ops + Context,
{
    for step in actions {
        match step {
            Action::File(filename) => executor.copy_file(filename)?,
            Action::Files(pattern) => executor.copy_files(pattern)?,
            Action::Context(context, sub_actions) => {
                let mut sub_contexts = Vec::new();
                match context {
                    Folder::Home => sub_contexts.push(executor.home()),
                    Folder::Config => sub_contexts.push(executor.config()),
                    Folder::Custom(name) => sub_contexts.push(executor.sub(name)),
                    Folder::Search(pattern) => sub_contexts.extend(executor.search(pattern)),
                }

                for sub_context in sub_contexts {
                    execute_actions(&sub_context, &sub_actions)?;
                }
            }
            Action::Execute(command, file_name) => executor.execute(command, file_name)?,
        }
    }
    Ok(())
}

pub fn execute<E: Ops + Context + Default>(
    executor: &E,
    app: &crate::App,
) -> Result<(), VacuumError> {
    execute_actions(executor, &app.actions)
}
