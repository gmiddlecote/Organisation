// import crossterm
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

// import the person library
use main_library::employee as lib_employee;
use main_library::person as lib_person;

// import ratatui
use ratatui::{prelude::*, widgets::*};

// import Result which the backend returns
use std::io::{stdout, Result};

/// # Usage
///
/// ```rust
/// let rect = centered_rect(f.size(), 50, 50);
/// ```
fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn main() -> Result<()> {
    // start ratatui
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    // end of start ratatui

    let first_person = first_employee();

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
            let new_para_area = centered_rect(frame.size(), 80, 20);
            frame.render_widget(
                Paragraph::new(first_person.to_string())
                    .white()
                    .on_cyan()
                    .wrap(Wrap { trim: true }),
                new_para_area,
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

fn first_employee() -> String {
    // Employee 1
    let emp_1_contact_number = vec![lib_person::ContactNumber {
        category: lib_person::ContactNumberCategory::Personal,
        number: "9988332277".to_string(),
    }];
    let emp_1 = lib_person::Person::new(
        "A",
        "",
        "C",
        "27-06-1974",
        lib_person::Gender::Male,
        lib_person::BloodType::A,
        lib_person::RHFactor::Positive,
        150.0,
        78.0,
        "IND".to_string(),
        lib_person::MaritalStatus::Single,
        "ANC43522",
        Vec::new(),
        emp_1_contact_number,
    );
    let employee_data = format!(
        "{} {:?} {:?} {:?} {} {} years {:?} {:?} {} {} {:?} {:?} {:?} {:?}",
        emp_1.first_name,
        emp_1.middle_name,
        emp_1.last_name,
        emp_1.emp_id,
        emp_1.dob,
        emp_1.get_current_age(),
        emp_1.gender,
        emp_1.complete_bloodtype,
        emp_1.height,
        emp_1.weight,
        emp_1.citizenship_country,
        emp_1.marital_status,
        emp_1.government_issue_id,
        emp_1.citizenship_country,
    );
    employee_data
}

fn _check() {
    // Employee 2
    let emp_2_contact_number = vec![lib_person::ContactNumber {
        category: lib_person::ContactNumberCategory::Personal,
        number: "9988332277".to_string(),
    }];
    let emp_2 = lib_person::Person::new(
        "X",
        "",
        "Z",
        "10-01-1950",
        lib_person::Gender::Female,
        lib_person::BloodType::B,
        lib_person::RHFactor::Positive,
        120.2,
        60.0,
        "AUS".to_string(),
        lib_person::MaritalStatus::Other("Reserved".to_string()),
        "giwhnf23442mkk2",
        Vec::new(),
        emp_2_contact_number,
    );
    println!(
        "{} {:?} {:?} {:?} {} {} years {:?} {:?} {} {} {:?} {:?} {:?}",
        emp_2.first_name,
        emp_2.middle_name,
        emp_2.last_name,
        emp_2.emp_id,
        emp_2.dob,
        emp_2.get_current_age(),
        emp_2.gender,
        emp_2.complete_bloodtype,
        emp_2.height,
        emp_2.weight,
        emp_2.citizenship_country,
        emp_2.marital_status,
        emp_2.government_issue_id,
    );
}
