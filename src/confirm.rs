use crate::{events::Event, value::DynamicValue};
use derive_builder::Builder;

use termion::event::Key::*;
use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders},
    Frame,
};

use crate::app::App;

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
pub struct ConfirmArgs {
    pub affirmative: Option<String>,
    pub negative: Option<String>,
    pub prompt: Option<String>,
}

impl App {
    pub fn confirm_render<B>(
        &mut self,
        f: &mut Frame<B>,
        size: Option<&Rect>,
        name: &str,
        args: ConfirmArgs,
    ) where
        B: tui::backend::Backend,
    {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Min(0)])
            .margin(1)
            .split(*size.unwrap_or(&f.size()));

        let block = Block::default()
            .title(
                args.prompt
                    .unwrap_or(String::from("Would you like to continue?")),
            )
            .borders(Borders::ALL);

        let buttons_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Length(5), Constraint::Min(0)])
            .split(block.inner(chunks[1]));

        let yes_style = Style::default().fg(Color::White).bg(Color::Green);

        let no_style = Style::default()
            .fg(Color::White)
            .bg(Color::Red)
            .add_modifier(Modifier::BOLD);
        let selected = self
            .map
            .get(&format!("{name}:selected"))
            .unwrap_or(&DynamicValue::Boolean(false));
        (if let DynamicValue::Boolean(selected) = selected && *selected {
            yes_style
        } else {
            no_style
        }).add_modifier(Modifier::BOLD);
        let yes = Block::default().title(vec![Span::styled(
            format!(" {} ", args.affirmative.unwrap_or(String::from("Yes"))),
            yes_style,
        )]);
        let no = Block::default().title(vec![Span::styled(
            format!(" {} ", args.negative.unwrap_or(String::from("No"))),
            no_style,
        )]);

        f.render_widget(yes, buttons_chunks[1]);

        f.render_widget(no, buttons_chunks[0]);
        f.render_widget(block, chunks[0]);
    }

    pub fn confirm_event(&mut self, name: &str) -> Option<bool> {
        #[cfg(feature = "termion")]
        return {
            if let Event::Input(key) = self.events.recv().unwrap() {
                match key {
                    Right => {
                        self.map
                            .insert(format!("{name}:selected"), DynamicValue::Boolean(true));
                        None
                    }
                    Left => {
                        self.map
                            .insert(format!("{name}:selected"), DynamicValue::Boolean(false));
                        None
                    }
                    Char('\n') => Some({
                        match self.map.get(&format!("{name}:selected")) {
                            Some(DynamicValue::Boolean(bool)) => *bool,
                            _ => unreachable!(),
                        }
                    }),
                    value => panic!("{value:?}"),
                }
            } else {
                None
            }
        };
    }
}
