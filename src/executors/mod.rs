mod context;
mod executor;

pub use context::Context;
pub use executor::Ops;

use crate::{Action, Folder};
use std::path::PathBuf;

pub type FileSystemExecutor = ContextPair<PathBuf>;
pub(crate) type ContextPair<E> = (E, E);

fn execute_actions<E>(executor: E, actions: &[Action])
where
    E: Ops + Context,
{
    for step in actions {
        for _ in 0..level {
            print!("  ");
        }
        match step {
            Action::Copy(filename) => executor.copy(filename),
            Action::CopyGlob(pattern) => executor.copy_glob(pattern),
            Action::Context(context, sub_actions) => {
                let contexes = match context {
                    Folder::Home => vec![executor.home()],
                    Folder::Config => vec![executor.config()],
                    Folder::Custom(name) => vec![executor.sub(name)],
                    Folder::Search(pattern) => executor.search(pattern),
                };

                for sub_context in contexes {
                    //let sub_executor = executor.sub(sub_context);
                    execute_actions(sub_context, &sub_actions, level + 1);
                }
            }
            Action::Execute(command) => executor.execute(command),
        }
    }
}

pub fn execute<E: Ops + Context + Default>(app: &crate::App) {
    println!("executing {}", app.name);
    let executor = E::default();

    execute_actions(executor, &app.actions, 0);
}
