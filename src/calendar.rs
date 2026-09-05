use crate::{read_date, read_text, read_time_range};
use chrono::{NaiveDate, NaiveTime};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fmt::Display,
    fs,
    io::{self, Write},
    path::PathBuf,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Calendar {
    notes: Vec<Note>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Note {
    day: NaiveDate,
    time: Option<TimeRange>,
    text: String,
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.day)?;

        match &self.time {
            None => writeln!(f, "   {}", self.text),
            Some(t) => writeln!(f, "   {} | {}", t, self.text),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeRange {
    pub start: NaiveTime,
    pub end: NaiveTime,
}

impl Display for TimeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {}",
            self.start.format("%H:%M"),
            self.end.format("%H:%M")
        )
    }
}

impl Calendar {
    fn get_path() -> PathBuf {
        let dir: PathBuf = match home_dir() {
            None => panic!("no home folder found"),
            Some(home) => home.join(".tuilar-rs"),
        };

        let path: PathBuf = dir.join("calendar.json");
        path
    }

    fn load() -> Result<Option<Self>, &'static str> {
        let path: PathBuf = Calendar::get_path();
        if !path.exists() {
            return Ok(None);
        }

        let content: String =
            fs::read_to_string(path).map_err(|_| "failed to read the contents of the file")?;

        match serde_json::from_str(&content) {
            Ok(cal) => return Ok(Some(cal)),
            Err(_) => return Err("failed to deserialize the content of the file"),
        }
    }

    pub fn add_note() -> Result<(), &'static str> {
        let path: PathBuf = Calendar::get_path();
        let mut cal: Calendar = match Calendar::load() {
            Ok(None) => {
                match fs::create_dir_all(path.parent().unwrap()) {
                    Ok(_) => {}
                    Err(_) => return Err("failed to initialize the calendar's directory"),
                }
                Calendar { notes: vec![] }
            }
            Ok(Some(cal)) => cal,
            Err(e) => return Err(e),
        };

        let date: NaiveDate = read_date();

        let timerange: Option<TimeRange> = read_time_range();

        let text: String = read_text();

        let new_note: Note = Note {
            day: date,
            time: timerange,
            text,
        };

        let pos = cal
            .notes
            .binary_search_by(|n| n.day.cmp(&date))
            .unwrap_or_else(|i| i);
        cal.notes.insert(pos, new_note);

        let json =
            serde_json::to_string_pretty(&cal).map_err(|_| "failed to serialize the calendar")?;
        fs::write(path, json).map_err(|_| "failed to save changes")?;

        Ok(())
    }

    pub fn read_notes() -> Result<(), &'static str> {
        let cal: Calendar = match Calendar::load() {
            Ok(None) => {
                println!("no calendar file found");
                println!("want to create a new note [y/n]?:");

                loop {
                    print!(">");
                    io::stdout().flush().unwrap();

                    let mut choice: String = String::new();
                    io::stdin()
                        .read_line(&mut choice)
                        .expect("should have been able to read");
                    let choice: &str = choice.trim();

                    match choice {
                        "y" | "yes" => match Calendar::add_note() {
                            Ok(_) => return Ok(()),
                            Err(e) => return Err(e),
                        },
                        "n" | "no" | "nope" => return Ok(()),
                        _ => {
                            println!("unknown option");
                            continue;
                        }
                    };
                }
            }
            Ok(Some(c)) => c,
            Err(e) => return Err(e),
        };

        for note in &cal.notes {
            println!("{}", note)
        }
        Ok(())
    }
}
