use ratatui::prelude::Stylize;
use ratatui::{DefaultTerminal, style::Style, text::Text};
fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let _app = run(&mut terminal);
    ratatui::restore();
    Ok(())
}

fn run(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    terminal.draw(|frame| {
        frame.render_widget(
            Text::from("hey man!").style(Style::new().red()),
            frame.area(),
        );
    })?;

    std::thread::sleep(std::time::Duration::from_secs(2));
    Ok(())
}
