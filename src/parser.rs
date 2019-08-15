use crate::app::{Action, App, Folder};
use pom::parser::{call, list, none_of, one_of, seq, sym};
use pom::Parser;

fn space() -> Parser<u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

fn string() -> Parser<u8, String> {
    let char_string = none_of(b"\\\"").repeat(0..).convert(String::from_utf8);
    sym(b'\"') * char_string - sym(b'\"')
}

fn command_copy() -> Parser<u8, Action> {
    (seq(b"copy") * space() * string()).map(Action::Copy)
}

fn command_copy_glob() -> Parser<u8, Action> {
    (seq(b"copy_glob") * space() * string()).map(Action::CopyGlob)
}

fn command_exec() -> Parser<u8, Action> {
    let command = seq(b"execute") | seq(b"exec");
    (command * space() * string()).map(Action::Execute)
}

fn context_home() -> Parser<u8, Action> {
    seq(b"home")
        * space()
        * call(actions)
            .map(|actions| Action::Context(Folder::Home, actions))
            .name("home")
}

fn context_config() -> Parser<u8, Action> {
    seq(b"config")
        * space()
        * call(actions)
            .map(|actions| Action::Context(Folder::Config, actions))
            .name("config")
}

fn context_search() -> Parser<u8, Action> {
    let f = seq(b"search") * space() * string() + space() * call(actions).name("search");
    f.map(|(pattern, actions)| Action::Context(Folder::Search(pattern), actions))
}

fn context_custom() -> Parser<u8, Action> {
    let f = seq(b"cd") * space() * string() + space() * call(actions).name("custom");
    f.map(|(folder, actions)| Action::Context(Folder::Custom(folder), actions))
}

fn actions() -> Parser<u8, Vec<Action>> {
    let item = command_copy()
        | command_copy_glob()
        | command_exec()
        | context_home()
        | context_config()
        | context_search()
        | context_custom();

    let items = list(item, sym(b';').opt() * space());
    let actions = sym(b'{') * space() * items - space() * sym(b'}');
    actions.name("actions")
}

fn parse_app() -> Parser<u8, App> {
    let app = space() * seq(b"app") * space() * string() + space() * call(actions);
    app.map(|(name, actions)| App { name, actions })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::Action;
    #[test]
    fn test_parse_context_custom() {
        let r = context_custom().parse(
            br#"cd "WebStorm" { 
            copy "*.xml"; 
            exec "ls files" 
        }"#,
        );

        assert_eq!(
            r,
            Ok(Action::Context(
                Folder::Custom("WebStorm".into()),
                vec![
                    Action::Copy("*.xml".into()),
                    Action::Execute("ls files".into())
                ]
            ))
        )
    }

    #[test]
    fn test_parse_context_search() {
        let r = context_search().parse(
            br#"search ".WebStorm*" { 
            copy "*.xml"; 
            exec "ls files"
        }"#,
        );

        assert_eq!(
            r,
            Ok(Action::Context(
                Folder::Search(".WebStorm*".into()),
                vec![
                    Action::Copy("*.xml".into()),
                    Action::Execute("ls files".into())
                ]
            ))
        )
    }

    #[test]
    fn test_parse_context_home() {
        let r = context_home().parse(
            br#"home { 
            copy "*.xml"; 
            exec "ls files" 
        }"#,
        );

        assert_eq!(
            r,
            Ok(Action::Context(
                Folder::Home,
                vec![
                    Action::Copy("*.xml".into()),
                    Action::Execute("ls files".into())
                ]
            ))
        )
    }

    #[test]
    fn test_parse_actions() {
        let r = actions().parse(
            br#"{ 
            copy "*.xml"
            exec "ls files" 
        }"#,
        );

        assert_eq!(
            r,
            Ok(vec![
                Action::Copy("*.xml".into()),
                Action::Execute("ls files".into())
            ])
        )
    }

    #[test]
    fn test_parse_copy() {
        let r = command_copy().parse(br#"copy "keyboard.xml""#);
        assert_eq!(r, Ok(Action::Copy("keyboard.xml".into())))
    }

    #[test]
    fn test_parse_copy_glob() {
        let r = command_copy_glob().parse(br#"copy_glob "*.xml""#);
        assert_eq!(r, Ok(Action::CopyGlob("*.xml".into())))
    }

    #[test]
    fn test_parse_execute() {
        let r = command_exec().parse(br#"exec "ls home""#);
        assert_eq!(r, Ok(Action::Execute("ls home".into())))
    }

    #[test]
    fn test_parse_app() {
        let r = parse_app().parse(
            br#"
            app "webstorm" {
                home {
                    search ".WebStorm*" {
                        cd "config" {
                            cd "keymaps" {
                                copy_glob "*.xml"
                            }
                            cd "options" {
                                copy "editor.xml"
                            }
                        }
                    }
                }
            }"#,
        );
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
                                    vec![Action::CopyGlob("*.xml".into())],
                                ),
                                Action::Context(
                                    Folder::Custom("options".into()),
                                    vec![Action::Copy("editor.xml".into())],
                                ),
                            ],
                        )],
                    )],
                )],
            })
        );
    }
}
