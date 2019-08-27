mod file_system_executor;
mod logger;
mod ops;

use crate::context::Context;
use crate::error::VacuumError;
use crate::{Action, Folder};
pub use file_system_executor::FileSystemExecutor;
pub use ops::Ops;

fn execute_actions<C>(
    executor: &impl Ops<Context = C>,
    ctx: &C,
    actions: &[Action],
) -> Result<(), VacuumError>
where
    C: Context,
{
    for step in actions {
        match step {
            Action::File(filename) => executor.copy_file(ctx, filename)?,
            Action::Files(pattern) => executor.copy_files(ctx, pattern)?,
            Action::Context(context, sub_actions) => {
                let mut sub_contexts = Vec::new();
                match context {
                    Folder::Home => sub_contexts.push(ctx.home()),
                    Folder::Config => sub_contexts.push(ctx.config()),
                    Folder::Custom(name) => sub_contexts.push(ctx.sub(name)),
                    Folder::Search(pattern) => sub_contexts.extend(ctx.search(pattern)),
                }

                for sub_context in sub_contexts {
                    execute_actions(executor, &sub_context, &sub_actions)?;
                }
            }
            Action::Execute(command, file_name) => executor.execute(ctx, command, file_name)?,
        }
    }
    Ok(())
}

pub fn execute<C>(
    executor: &impl Ops<Context = C>,
    ctx: &C,
    app: &crate::App,
) -> Result<(), VacuumError>
where
    C: Context,
{
    execute_actions(executor, ctx, &app.actions)
}
