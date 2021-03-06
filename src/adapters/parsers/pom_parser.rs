use crate::application::error::VacuumError;
use crate::application::parser::VacuumFileParser;
use crate::domain::{Action, App, Dependency, DependencyCheck, Folder};
use pom::parser::*;
use std::iter::FromIterator;

pub struct PomParser;

impl VacuumFileParser for PomParser {
    fn parse(input: String) -> Result<App, VacuumError> {
        let input = input.chars().collect::<Vec<_>>();
        let result = parse_vacuum_file().parse(&input);
        result.map_err(VacuumError::ParseError)
    }
}

fn space<'a>() -> Parser<'a, char, ()> {
    one_of(" \t\r\n").repeat(0..).discard()
}

fn ident<'a>() -> Parser<'a, char, String> {
    none_of("\\\", \r\n\t[]{}()")
        .repeat(1..)
        .map(String::from_iter)
}

fn string<'a>() -> Parser<'a, char, String> {
    let char_string = none_of("\\\"").repeat(0..).map(String::from_iter);
    sym('\"') * char_string - sym('\"')
}

fn dependency_exists<'a>() -> Parser<'a, char, DependencyCheck> {
    (tag("exists") * space() * tag("->") * space() * ident() - space())
        .name("dependency_exists")
        .map(DependencyCheck::Exists)
}

fn dependency_contains<'a>() -> Parser<'a, char, DependencyCheck> {
    (tag("contains") * space() * string() + space() * tag("->") * space() * ident() - space())
        .name("dependency_contains")
        .map(|(pattern, dep_name)| DependencyCheck::Contains(pattern, dep_name))
}

fn dependencies<'a>() -> Parser<'a, char, Vec<DependencyCheck>> {
    let dependency_rules = dependency_exists() | dependency_contains();

    let items = list(dependency_rules, sym(',') * space());
    let dependencies = space() * sym('[') * space() * items - space() * sym(']');
    dependencies.name("dependencies")
}

fn command_file<'a>() -> Parser<'a, char, Action> {
    (tag("file") * space() * string() + space() * dependencies().opt())
        .map(|(f, d)| Action::File(f, d))
}

fn command_files<'a>() -> Parser<'a, char, Action> {
    (tag("files") * space() * string()).map(Action::Files)
}

fn command_exec<'a>() -> Parser<'a, char, Action> {
    let command = tag("execute") | tag("exec");
    let command = command * space() * string() + (space() * tag(">>") * space() * string()).opt();

    command.map(|(command, file_name)| Action::Execute(command, file_name))
}

fn context_home<'a>() -> Parser<'a, char, Action> {
    tag("home")
        * space()
        * call(parse_actions)
            .map(|actions| Action::Context(Folder::Home, actions))
            .name("home")
}

fn context_config<'a>() -> Parser<'a, char, Action> {
    tag("config")
        * space()
        * call(parse_actions)
            .map(|actions| Action::Context(Folder::Config, actions))
            .name("config")
}

fn context_local<'a>() -> Parser<'a, char, Action> {
    tag("local")
        * space()
        * call(parse_actions)
            .map(|actions| Action::Context(Folder::Local, actions))
            .name("local")
}

fn context_search<'a>() -> Parser<'a, char, Action> {
    let f = tag("search") * space() * string() + space() * call(parse_actions).name("search");
    f.map(|(pattern, actions)| Action::Context(Folder::Search(pattern), actions))
}

fn context_custom<'a>() -> Parser<'a, char, Action> {
    let f = tag("cd") * space() * string() + space() * call(parse_actions).name("custom");
    f.map(|(folder, actions)| Action::Context(Folder::Custom(folder), actions))
}

fn parse_actions<'a>() -> Parser<'a, char, Vec<Action>> {
    let item = command_file()
        | command_files()
        | command_exec()
        | context_home()
        | context_config()
        | context_local()
        | context_search()
        | context_custom();

    let items = list(item, sym(';').opt() * space());
    let actions = sym('{') * space() * items - space() * sym('}');
    actions.name("actions")
}

fn dependency_rule<'a>() -> Parser<'a, char, Dependency> {
    let block = none_of("}").repeat(1..).map(String::from_iter);

    (space() * ident() + space() * sym('{') * block - space() * sym('}'))
        .map(|(name, block)| Dependency { name, block })
        .name("dependency_rule")
}

fn parse_dependencies_section<'a>() -> Parser<'a, char, Vec<Dependency>> {
    let dependency_rules = list(dependency_rule(), space()).name("dependency_rules");
    let dependencies_section =
        space() * tag("dependencies") * space() * sym('{') * dependency_rules - space() * sym('}');

    dependencies_section.name("dependencies_section")
}

fn parse_vacuum_file<'a>() -> Parser<'a, char, App> {
    let app = space() * tag("app") * space() * string()
        + space() * call(parse_actions)
        + parse_dependencies_section().opt();

    app.map(|((name, actions), dependencies)| App {
        name,
        actions,
        dependencies,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Action;

    #[test]
    fn test_parse_dependency_exists() {
        let input = r#"exists     ->   rule   , contains"#.chars().collect::<Vec<_>>();
        let r = dependency_exists().parse(&input).unwrap();

        assert_eq!(r, DependencyCheck::Exists("rule".into()))
    }

    #[test]
    fn test_parse_dependency_contains() {
        let input = r#"contains  "content"  ->  dep,exists "#
            .chars()
            .collect::<Vec<_>>();
        let r = dependency_contains().parse(&input).unwrap();

        assert_eq!(r, DependencyCheck::Contains("content".into(), "dep".into()))
    }

    #[test]
    fn test_parse_dependencies() {
        let input = r#"[exists -> dep1   , contains "content" -> dep2]"#
            .chars()
            .collect::<Vec<_>>();
        let r = dependencies().parse(&input).unwrap();

        assert_eq!(
            r,
            vec![
                DependencyCheck::Exists("dep1".into()),
                DependencyCheck::Contains("content".into(), "dep2".into())
            ]
        )
    }

    #[test]
    fn test_parse_dependency_rule() {
        let input = r#"
           
                dep1 {
                    curl -L http://application.com
                }
          
        "#
        .chars()
        .collect::<Vec<_>>();
        let r = dependency_rule().parse(&input).unwrap();

        assert_eq!(
            r,
            Dependency {
                name: "dep1".into(),
                block: r#"
                    curl -L http://application.com
                "#
                .into(),
            }
        )
    }

    #[test]
    fn test_parse_dependencies_section() {
        let input = r#"
            dependencies {
                dep1 {
                    curl -L http://application.com
                }
                dep2 {
                    curl -L http://application2.com
                }
            }
        "#
        .chars()
        .collect::<Vec<_>>();
        let r = parse_dependencies_section().parse(&input).unwrap();

        assert_eq!(
            r,
            vec![
                Dependency {
                    name: "dep1".into(),
                    block: r#"
                    curl -L http://application.com
                "#
                    .into(),
                },
                Dependency {
                    name: "dep2".into(),
                    block: r#"
                    curl -L http://application2.com
                "#
                    .into(),
                }
            ]
        )
    }

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
                    Action::File("*.xml".into(), None),
                    Action::Execute("ls files".into(), None)
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
                    Action::File("*.xml".into(), None),
                    Action::Execute("ls files".into(), None)
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
                    Action::File("*.xml".into(), None),
                    Action::Execute("ls files".into(), None)
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

        let r = parse_actions().parse(&input);
        assert_eq!(
            r,
            Ok(vec![
                Action::File("*.xml".into(), None),
                Action::Execute("ls files".into(), None)
            ])
        )
    }

    #[test]
    fn test_parse_file() {
        let input = r#"file "keyboard.xml""#.chars().collect::<Vec<_>>();
        let r = command_file().parse(&input);
        assert_eq!(r, Ok(Action::File("keyboard.xml".into(), None)))
    }

    #[test]
    fn test_parse_file_with_dependencies() {
        let input = r#"file "keyboard.xml" [exists -> dep1 , contains "plug" -> dep2 ]"#
            .chars()
            .collect::<Vec<_>>();

        let r = command_file().parse(&input);
        assert_eq!(
            r,
            Ok(Action::File(
                "keyboard.xml".into(),
                Some(vec![
                    DependencyCheck::Exists("dep1".into()),
                    DependencyCheck::Contains("plug".into(), "dep2".into())
                ]),
            ))
        )
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
        assert_eq!(r, Ok(Action::Execute("ls home".into(), None)))
    }

    #[test]
    fn test_parse_execute_with_capture() {
        let input = r#"exec "ls home" >> "output.txt""#.chars().collect::<Vec<_>>();
        let r = command_exec().parse(&input);
        assert_eq!(
            r,
            Ok(Action::Execute("ls home".into(), Some("output.txt".into())))
        )
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

        let r = parse_vacuum_file().parse(&input);
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
                                    vec![Action::File("editor.xml".into(), None)],
                                ),
                            ],
                        )],
                    )],
                )],
                dependencies: None
            })
        );
    }

    #[test]
    fn test_parse_deps() {
        let input = r#"
            app "some-app" {
                home {
                    file "some-app.config" [exists -> dep1, contains "content" -> dep2]
                }
            }
            
            dependencies {
                dep1 {
                    curl -L http://application.com
                }
                dep2 {
                    curl -L http://application2.com
                }
            }
            "#
        .chars()
        .collect::<Vec<_>>();

        let r = parse_vacuum_file().parse(&input);
        assert_eq!(
            r,
            Ok(App {
                name: "some-app".into(),
                actions: vec![Action::Context(
                    Folder::Home,
                    vec![Action::File(
                        "some-app.config".into(),
                        Some(vec![
                            DependencyCheck::Exists("dep1".into()),
                            DependencyCheck::Contains("content".into(), "dep2".into())
                        ]),
                    )],
                )],
                dependencies: Some(vec![
                    Dependency {
                        name: "dep1".into(),
                        block: r#"
                    curl -L http://application.com
                "#
                        .into(),
                    },
                    Dependency {
                        name: "dep2".into(),
                        block: r#"
                    curl -L http://application2.com
                "#
                        .into(),
                    }
                ])
            })
        );
    }
}
