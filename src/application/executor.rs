use crate::application::context::Context;
use crate::application::error::VacuumError;
use crate::application::handler::Handler;
use crate::domain::{Action, App, Folder};

fn handle_actions<C>(
    handler: &impl Handler<Context = C>,
    ctx: &C,
    actions: &[Action],
) -> Result<(), VacuumError>
where
    C: Context,
{
    for step in actions {
        match step {
            Action::File(filename) => handler.handle_file(ctx, filename)?,
            Action::FileWithDependencies(filename, _) => handler.handle_file(ctx, filename)?,
            Action::Files(pattern) => handler.handle_files(ctx, pattern)?,
            Action::Context(context, sub_actions) => {
                let mut sub_contexts = Vec::new();
                match context {
                    Folder::Home => sub_contexts.push(ctx.home()),
                    Folder::Config => sub_contexts.push(ctx.config()),
                    Folder::Custom(name) => sub_contexts.push(ctx.sub(name)),
                    Folder::Search(pattern) => sub_contexts.extend(ctx.search(pattern)),
                }

                for sub_context in sub_contexts {
                    handle_actions(handler, &sub_context, &sub_actions)?;
                }
            }
            Action::Execute(command, file_name) => {
                handler.handle_execute(ctx, command, file_name)?
            }
        }
    }
    Ok(())
}

pub fn execute<C>(
    handler: &impl Handler<Context = C>,
    ctx: &C,
    app: &App,
) -> Result<(), VacuumError>
where
    C: Context,
{
    handle_actions(handler, ctx, &app.actions)
}
