use core::panic;
use std::{cmp, fs, path::Path};

use serde::Deserialize;
use tui_realm_stdlib::Table;
use tuirealm::{
    command::{Cmd, CmdResult, Direction, Position},
    event::{Key, KeyEvent},
    props::{Alignment, BorderSides, Borders, TableBuilder, TextSpan},
    Component, Event, MockComponent, NoUserEvent,
};

use crate::project_picker::Msg;

#[derive(MockComponent)]
pub struct ProjectPicker {
    component: Table,
    cur_item: usize,
    projects: Vec<Project>,
    last_item: usize,
}

#[derive(Debug)]
#[allow(dead_code)]
enum ProjectPickerError {
    Config(String),
}

#[derive(Deserialize, Debug)]
struct Project {
    name: String,
}

impl ProjectPicker {
    fn load_projects(cwd: &str) -> Vec<Project> {
        let path = Path::new(cwd);
        let project_paths = fs::read_dir(&path).map(|files| {
            files
                .into_iter()
                .map(|file_reader| {
                    file_reader
                        .map(|file| match file.path().into_os_string().into_string() {
                            Ok(name) => Some(Project {
                                name: name.replace(cwd, ""),
                            }),
                            Err(_) => None,
                        })
                        .unwrap_or(None)
                })
                .filter_map(|p| p)
                .collect::<Vec<Project>>()
        });

        project_paths.expect("Failed to get projects")
    }
}

impl Default for ProjectPicker {
    fn default() -> Self {
        let highlight = tuirealm::props::Color::Rgb(125, 94, 144);
        let text_passive = tuirealm::props::Color::Rgb(146, 108, 170);
        let default_cwd = "/Users/dnlmrtnz/Projects/";
        let data = ProjectPicker::load_projects(default_cwd);

        let mut table_builder = TableBuilder::default();

        let mut cur = 0;
        let last_index = data.len();

        for row in &data {
            table_builder.add_col(TextSpan::from(&row.name).fg(text_passive));

            cur += 1;
            if cur != last_index {
                table_builder.add_row();
            }
        }

        let cur_item = 0;

        Self {
            component: Table::default()
                .borders(Borders::default().sides(BorderSides::empty()))
                .title(format!("Projects ({})", default_cwd), Alignment::Center)
                .scroll(true)
                .selected_line(cur_item)
                .highlighted_color(highlight)
                .rewind(true)
                .step(4)
                .row_height(1)
                .headers(&["Name"])
                .column_spacing(3)
                .widths(&[30, 70])
                .table(table_builder.build()),
            cur_item,
            projects: data,
            last_item: last_index - 1,
        }
    }
}

impl Component<Msg, NoUserEvent> for ProjectPicker {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent {
                                code: Key::Down, ..
                            }) => {
                self.cur_item += cmp::max(self.last_item, self.cur_item + 1);
                self.perform(Cmd::Move(Direction::Down))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('j'),
                ..
            }) => {
                self.cur_item = cmp::min(self.last_item, self.cur_item + 1);
                self.perform(Cmd::Move(Direction::Down))
            }
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.cur_item = cmp::min(0, self.cur_item - 1);
                self.perform(Cmd::Move(Direction::Up))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('k'),
                ..
            }) => {
                self.cur_item = cmp::min(0, self.cur_item - 1);
                self.perform(Cmd::Move(Direction::Up))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => {
                self.cur_item = 0;
                self.perform(Cmd::GoTo(Position::Begin))
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.cur_item = self.last_item;
                self.perform(Cmd::GoTo(Position::End))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('q'),
                ..
            }) => return Some(Msg::AppClose),

            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => match self.projects.get(self.cur_item) {
                Some(Project { name }) => return Some(Msg::SelectProject(String::from(name))),
                None => panic!("Selected project does not exist"),
            },
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}
