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
    fn get_path() -> Option<PathBuf> {
        let dir: PathBuf = match home_dir() {
            None => panic!("no home folder found"),
            Some(home) => home.join(".tuilar-rs"),
        };

        let path: PathBuf = dir.join("calendar.json");

        if path.exists() {
            return Some(path);
        }
        None
    }

    fn load() -> Result<Self, &'static str> {
        let path: PathBuf = match Calendar::get_path() {
            None => return Ok(Calendar { notes: vec![] }),
            Some(p) => p,
        };

        let content: String =
            fs::read_to_string(path).map_err(|_| "failed to read the contents of the file")?;

        match serde_json::from_str(&content) {
            Ok(cal) => return Ok(cal),
            Err(_) => return Err("failed to deserialize the content of the file"),
        }
    }
}

fn main() {
    let date: NaiveDate = read_date();
    dbg!(date);

    let x = read_time_range();
    dbg!(x);

    let y = read_text();
    println!("{y}")
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
