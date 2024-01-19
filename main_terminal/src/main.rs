use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use lib_employees::*;
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, IsTerminal, Result};

fn main() -> Result<()> {
    // start ratatui
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    // end of start ratatui

    // start of main loop to draw to the terminal
    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hello (press 'q' to quit)")
                    .white()
                    .on_blue(),
                area,
            );
        })?;

        // check 'q' to quit the loop
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }
    // end of draw loop

    // exit ratatui
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
    // end of exit ratatui
}

fn _check() {
    let emp_1 = Person::new("A", "B", "C", "27-06-1974", Gender::Male);
    let emp_2 = Person::new("X", "", "Z", "10-01-1950", Gender::Female);
    println!(
        "{} {:?} {:?} {:?} {} {} years {:?}",
        emp_1.first_name,
        emp_1.middle_name,
        emp_1.last_name,
        emp_1.emp_id,
        emp_1.dob.to_string(),
        emp_1.get_current_age(),
        emp_1.gender
    );
    println!(
        "{} {:?} {:?} {:?} {} {} years",
        emp_2.first_name,
        emp_2.middle_name,
        emp_2.last_name,
        emp_2.emp_id,
        emp_2.dob.to_string(),
        emp_2.get_current_age()
    );
}
