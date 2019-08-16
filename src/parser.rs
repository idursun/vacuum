use crate::app::{Action, App, Folder};
use pom::parser::*;
use std::iter::FromIterator;

fn space<'a>() -> Parser<'a, char, ()> {
    one_of(" \t\r\n").repeat(0..).discard()
}

fn string<'a>() -> Parser<'a, char, String> {
    let char_string = none_of("\\\"").repeat(0..).map(String::from_iter);
    sym('\"') * char_string - sym('\"')
}

fn command_file<'a>() -> Parser<'a, char, Action> {
    (tag("file") * space() * string()).map(Action::File)
}

fn command_files<'a>() -> Parser<'a, char, Action> {
    (tag("files") * space() * string()).map(Action::Files)
}

fn command_exec<'a>() -> Parser<'a, char, Action> {
    let command = tag("execute") | tag("exec");
    (command * space() * string()).map(Action::Execute)
}

fn context_home<'a>() -> Parser<'a, char, Action> {
    tag("home")
        * space()
        * call(actions)
            .map(|actions| Action::Context(Folder::Home, actions))
            .name("home")
}

fn context_config<'a>() -> Parser<'a, char, Action> {
    tag("config")
        * space()
        * call(actions)
            .map(|actions| Action::Context(Folder::Config, actions))
            .name("config")
}

fn context_search<'a>() -> Parser<'a, char, Action> {
    let f = tag("search") * space() * string() + space() * call(actions).name("search");
    f.map(|(pattern, actions)| Action::Context(Folder::Search(pattern), actions))
}

fn context_custom<'a>() -> Parser<'a, char, Action> {
    let f = tag("cd") * space() * string() + space() * call(actions).name("custom");
    f.map(|(folder, actions)| Action::Context(Folder::Custom(folder), actions))
}

fn actions<'a>() -> Parser<'a, char, Vec<Action>> {
    let item = command_file()
        | command_files()
        | command_exec()
        | context_home()
        | context_config()
        | context_search()
        | context_custom();

    let items = list(item, sym(';').opt() * space());
    let actions = sym('{') * space() * items - space() * sym('}');
    actions.name("actions")
}

fn parse_app<'a>() -> Parser<'a, char, App> {
    let app = space() * tag("app") * space() * string() + space() * call(actions);
    app.map(|(name, actions)| App { name, actions })
}

pub fn app(input: String) -> Result<App, Box<dyn std::error::Error>> {
    let input = input.chars().collect::<Vec<_>>();
    let result = parse_app().parse(&input);
    match result {
        Ok(app) => Ok(app),
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::Action;
    #[test]
    fn test_parse_context_custom() {
        let input = r#"cd "WebStorm" { 
            file "*.xml"; 
            exec "ls files" 
        }"#
        .chars()
        .collect::<Vec<_>>();
        let r = context_custom().parse(&input);

        assert_eq!(
            r,
            Ok(Action::Context(
                Folder::Custom("WebStorm".into()),
                vec![
                    Action::File("*.xml".into()),
                    Action::Execute("ls files".into())
                ]
            ))
        )
    }

    #[test]
    fn test_parse_context_search() {
        let input = r#"search ".WebStorm*" { 
            file "*.xml"; 
            exec "ls files"
        }"#
        .chars()
        .collect::<Vec<_>>();

        let r = context_search().parse(&input);
        assert_eq!(
            r,
            Ok(Action::Context(
                Folder::Search(".WebStorm*".into()),
                vec![
                    Action::File("*.xml".into()),
                    Action::Execute("ls files".into())
                ]
            ))
        )
    }

    #[test]
    fn test_parse_context_home() {
        let input = r#"home { 
            file "*.xml"; 
            exec "ls files" 
        }"#
        .chars()
        .collect::<Vec<_>>();

        let r = context_home().parse(&input);
        assert_eq!(
            r,
            Ok(Action::Context(
                Folder::Home,
                vec![
                    Action::File("*.xml".into()),
                    Action::Execute("ls files".into())
                ]
            ))
        )
    }

    #[test]
    fn test_parse_actions() {
        let input = &r#"{ 
            file "*.xml"
            exec "ls files" 
        }"#
        .chars()
        .collect::<Vec<_>>();

        let r = actions().parse(&input);
        assert_eq!(
            r,
            Ok(vec![
                Action::File("*.xml".into()),
                Action::Execute("ls files".into())
            ])
        )
    }

    #[test]
    fn test_parse_file() {
        let input = r#"file "keyboard.xml""#.chars().collect::<Vec<_>>();
        let r = command_file().parse(&input);
        assert_eq!(r, Ok(Action::File("keyboard.xml".into())))
    }

    #[test]
    fn test_parse_files() {
        let input = r#"files "*.xml""#.chars().collect::<Vec<_>>();
        let r = command_files().parse(&input);
        assert_eq!(r, Ok(Action::Files("*.xml".into())))
    }

    #[test]
    fn test_parse_execute() {
        let input = r#"exec "ls home""#.chars().collect::<Vec<_>>();
        let r = command_exec().parse(&input);
        assert_eq!(r, Ok(Action::Execute("ls home".into())))
    }

    #[test]
    fn test_parse_app() {
        let input = r#"
            app "webstorm" {
                home {
                    search ".WebStorm*" {
                        cd "config" {
                            cd "keymaps" {
                                files "*.xml"
                            }
                            cd "options" {
                                file "editor.xml"
                            }
                        }
                    }
                }
            }"#
        .chars()
        .collect::<Vec<_>>();

        let r = parse_app().parse(&input);
        assert_eq!(
            r,
            Ok(App {
                name: "webstorm".into(),
                actions: vec![Action::Context(
                    Folder::Home,
                    vec![Action::Context(
                        Folder::Search(".WebStorm*".into()),
                        vec![Action::Context(
                            Folder::Custom("config".into()),
                            vec![
                                Action::Context(
                                    Folder::Custom("keymaps".into()),
                                    vec![Action::Files("*.xml".into())],
                                ),
                                Action::Context(
                                    Folder::Custom("options".into()),
                                    vec![Action::File("editor.xml".into())],
                                ),
                            ],
                        )],
                    )],
                )],
            })
        );
    }
}
