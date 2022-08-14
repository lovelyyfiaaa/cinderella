use cfg_if::cfg_if;
use std::io;
use std::io::Stdout;
use termion::input::MouseTerminal;
use termion::screen::AlternateScreen;
use tui::Terminal;

pub mod app;
pub mod confirm;
pub mod events;
pub mod value;

cfg_if! {
    if #[cfg(feature = "crossterm")] {
        use tui::backend::CrosstermBackend;

    } else if #[cfg(feature = "termion")] {
        use termion::raw::IntoRawMode;
        use tui::backend::TermionBackend;

        use termion::raw::RawTerminal;
    }
}

cfg_if! {
    if #[cfg(feature = "termion")]{
        type InitReturn = Terminal<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>>;

        pub fn init_terminal() -> Result<InitReturn, std::io::Error> {
            let stdout = io::stdout().into_raw_mode()?;
            let stdout = MouseTerminal::from(stdout);
            let stdout = AlternateScreen::from(stdout);
            let backend = TermionBackend::new(stdout);
            let terminal = Terminal::new(backend)?;
            Ok(terminal)
        }
    } else if #[cfg(feature = "crossterm")]{
        pub fn init_terminal() {
            let backend = CrosstermBackend::new(stdout)?;

            let mut terminal = Terminal::new(backend)?;
            Ok(terminal)
        }
    }
}

#[cfg(test)]
mod tests {}
