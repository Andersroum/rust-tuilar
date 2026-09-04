use chrono::{NaiveDate, NaiveTime};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize)]
struct Calendar {
    notes: Vec<Note>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Note {
    day: NaiveDate,
    time: Option<TimeRange>,
    text: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TimeRange {
    start: NaiveTime,
    end: NaiveTime,
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

    fn add_note() -> Result<(), &'static str> {
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

        cal.notes.push(Note {
            day: date,
            time: timerange,
            text,
        });

        let json =
            serde_json::to_string_pretty(&cal).map_err(|_| "failed to serialize the calendar")?;
        fs::write(path, json).map_err(|_| "failed to save changes")?;

        Ok(())
    }
}

fn main() {
    println!("what do you want to do?");
    println!("1: add a note");

    print!(">");
    io::stdout().flush().unwrap();

    let mut choice: String = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("couldn't read your choice");

    let choice: &str = choice.trim();
    match choice {
        "1" => match Calendar::add_note() {
            Ok(()) => {}
            Err(e) => println!("{}", e),
        },
        _ => println!("unknown option"),
    }
}

fn read_date() -> NaiveDate {
    println!("enter date([YYYY-MM-DD]):");
    loop {
        print!(">");
        io::stdout().flush().unwrap();

        let mut date: String = String::new();
        io::stdin()
            .read_line(&mut date)
            .expect("failed to read the input");

        match NaiveDate::parse_from_str(&date.trim(), "%Y-%m-%d") {
            Ok(date) => return date,
            Err(_) => {
                println!("invalid date. Please try again");
                continue;
            }
        };
    }
}

fn read_time_range() -> Option<TimeRange> {
    let (start_time, end_time) = {
        println!("enter start time([HH:MM]) or leave empty for an all day task:");

        let start_time: NaiveTime = loop {
            print!(">");
            io::stdout().flush().unwrap();

            let mut start_time: String = String::new();
            io::stdin()
                .read_line(&mut start_time)
                .expect("should have been able to read");
            let start_time: &str = start_time.trim();

            if start_time.is_empty() {
                return None;
            }

            match NaiveTime::parse_from_str(start_time, "%H:%M") {
                Ok(time) => break time,
                Err(_) => {
                    println!("invalid time. Please try again");
                    continue;
                }
            }
        };

        println!("enter end time([HH:MM]) or leave empty");
        println!("to discard the start time and set as an all day task");

        let end_time: NaiveTime = loop {
            print!(">");
            io::stdout().flush().unwrap();

            let mut end_time: String = String::new();
            io::stdin()
                .read_line(&mut end_time)
                .expect("should have been able to read");
            let end_time: &str = end_time.trim();

            if end_time.is_empty() {
                return None;
            }

            match NaiveTime::parse_from_str(end_time, "%H:%M") {
                Ok(time) => {
                    if time < start_time {
                        println!("the entered end time is smaller than the start time.");
                        println!("Therefore it's invalid: please try again");
                        continue;
                    } else {
                        break time;
                    }
                }
                Err(_) => {
                    println!("invalid time. Please try again");
                    continue;
                }
            }
        };

        (start_time, end_time)
    };

    Some(TimeRange {
        start: start_time,
        end: end_time,
    })
}

fn read_text() -> String {
    println!("enter the note:");
    print!(">");
    io::stdout().flush().unwrap();

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("should have been able to read");
    let input: String = input.trim().to_string();

    input
}
