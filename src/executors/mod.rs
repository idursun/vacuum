mod context;
mod file_system_executor;
mod ops;

use crate::{Action, Folder};
pub use context::Context;
pub use file_system_executor::FileSystemExecutor;

pub use ops::Ops;

fn execute_actions<E>(executor: &E, actions: &[Action])
where
    E: Ops + Context,
{
    for step in actions {
        match step {
            Action::Copy(filename) => executor.copy(filename),
            Action::CopyGlob(pattern) => executor.copy_glob(pattern),
            Action::Context(context, sub_actions) => {
                let sub_contexts = match context {
                    Folder::Home => vec![executor.home()],
                    Folder::Config => vec![executor.config()],
                    Folder::Custom(name) => vec![executor.sub(name)],
                    Folder::Search(pattern) => executor.search(pattern),
                };

                for sub_context in sub_contexts {
                    //let sub_executor = executor.sub(sub_context);
                    execute_actions(&sub_context, &sub_actions);
                }
            }
            Action::Execute(command) => executor.execute(command),
        }
    }
}

pub fn execute<E: Ops + Context + Default>(executor: &E, app: &crate::App) {
    execute_actions(executor, &app.actions);
}
