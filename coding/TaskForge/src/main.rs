// src/main.rs

use rataui::{DefaultTerminal, Frame};

fn main() -> color_eyre::Result<()> {
    color_eyre::install();
    rataui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("Hello, World!", frame.area());
}