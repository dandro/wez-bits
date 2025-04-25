use tuirealm::{
    command::Cmd,
    event::{Key, KeyEvent, KeyModifiers},
    props::Style,
    ratatui::widgets::Paragraph,
    AttrValue, Attribute, Component, MockComponent, NoUserEvent, Props,
};

use crate::project_picker::Msg;

#[derive(Default)]
pub struct Heading {
    pub props: Props,
}

impl Heading {
    pub fn text<S>(mut self, s: S) -> Self
    where
        S: AsRef<str>,
    {
        self.attr(Attribute::Text, AttrValue::String(s.as_ref().to_string()));
        self
    }
}

impl MockComponent for Heading {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::ratatui::prelude::Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            let text = self
                .props
                .get_or(Attribute::Text, AttrValue::String(String::default()))
                .unwrap_string();

            let theme_green = tuirealm::props::Color::Rgb(205, 220, 57);

            frame.render_widget(
                Paragraph::new(text).style(Style::new().fg(theme_green)),
                area,
            );
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value)
    }

    fn state(&self) -> tuirealm::State {
        tuirealm::State::None
    }

    fn perform(&mut self, _cmd: tuirealm::command::Cmd) -> tuirealm::command::CmdResult {
        tuirealm::command::CmdResult::None
    }
}

impl Component<Msg, NoUserEvent> for Heading {
    fn on(&mut self, ev: tuirealm::Event<NoUserEvent>) -> Option<Msg> {
        let cmd: Cmd = match ev {
            tuirealm::Event::Keyboard(KeyEvent {
                code: Key::Esc,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::AppClose),
            _ => Cmd::None,
        };

        match self.perform(cmd) {
            _ => None,
        }
    }
}
