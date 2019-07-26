mod context;
mod ops;

pub use context::Context;
pub use ops::Ops;

use crate::{Action, Folder};
use std::path::PathBuf;

pub type FileSystemExecutor = ContextPair<PathBuf>;
pub(crate) type ContextPair<E> = (E, E);

pub fn file_system_executor(target_dir: PathBuf) -> FileSystemExecutor {
    (PathBuf::new(), target_dir)
}

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
    println!("executing {}", app.name);
    execute_actions(executor, &app.actions);
}
