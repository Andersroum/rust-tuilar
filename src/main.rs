use crate::calendar::{Calendar, TimeRange};
use chrono::{NaiveDate, NaiveTime};
use std::io::{self, Write};

mod calendar;

fn main() {
    println!("\nwhat do you want to do?");
    println!("1: add a note");
    println!("2: read your notes");

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
        "2" => match Calendar::read_notes() {
            Ok(_) => {}
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
