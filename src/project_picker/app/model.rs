use std::time::Duration;

use tuirealm::{
    ratatui::layout::{Constraint, Direction, Layout},
    terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalBridge},
    Application, EventListenerCfg, NoUserEvent, Update,
};

use crate::project_picker::{components::picker::ProjectPicker, Id, Msg};
use crate::wezterm;

pub struct Model<T>
where
    T: TerminalAdapter,
{
    pub app: Application<Id, Msg, NoUserEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: TerminalBridge<T>,
}

impl Default for Model<CrosstermTerminalAdapter> {
    fn default() -> Self {
        Model {
            app: Self::init_app(),
            quit: false,
            redraw: true,
            terminal: TerminalBridge::init_crossterm().expect("Could not initialize terminal"),
        }
    }
}

impl<T> Model<T>
where
    T: TerminalAdapter,
{
    pub fn view(&mut self) {
        assert!(self
            .terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(f.area());

                self.app.view(&Id::ProjectPicker, f, chunks[0]);
            })
            .is_ok());
    }

    fn init_app() -> Application<Id, Msg, NoUserEvent> {
        let mut app = Application::init(
            EventListenerCfg::default()
                .crossterm_input_listener(Duration::from_millis(20), 3)
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1)),
        );

        assert!(app
            .mount(
                Id::ProjectPicker,
                Box::new(ProjectPicker::default()),
                Vec::default()
            )
            .is_ok());

        // It is important to activate the component with the ESC listener!!
        assert!(app.active(&Id::ProjectPicker).is_ok());
        app
    }
}

impl<T> Update<Msg> for Model<T>
where
    T: TerminalAdapter,
{
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        if let Some(msg) = msg {
            self.redraw = true;
            match msg {
                Msg::AppClose => {
                    self.quit = true;
                    None
                }
                Msg::SelectProject(project) => {
                    // let result = Command::new("wezterm")
                    //     .args(&["start", "--", &format!("~/Projects/{}", project)])
                    //     .output();

                    let result = wezterm::open_pane(wezterm::Direction::Right, 70).and_then(|id| {
                        wezterm::pipe_stdout_to_pane(
                            vec![String::from("hx"), format!("~/Projects/{}", project)],
                            id,
                        )
                    });

                    match result {
                        Ok(_) => None,
                        Err(_) => None,
                    }
                }
                Msg::None => None,
            }
        } else {
            None
        }
    }
}
