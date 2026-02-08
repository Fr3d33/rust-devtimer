use std::env;
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json;
use chrono::Local;

#[derive(Serialize, Deserialize)]
struct SavedTime {
    text: String,
    timestamp_start: String,
    timestamp_stop: Option<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let now = Local::now();
    let datetime = now.format("%Y-%m-%d %H:%M:%S").to_string();

    if args.len() < 2 {
        println!("
DevTimer - Simple CLI Time Tracking Tool

USAGE:
    devtimer <COMMAND> [ARGS]

COMMANDS:
    start <description>    Start tracking time with a description
    stop                   Stop the current timer
    statistics             Display statistics for the last tracked entry

EXAMPLES:
    devtimer start Working on feature X
    devtimer stop
    devtimer statistics");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "start" => {
            if args.len() < 3 {
                println!("Error: Please provide a description for the timer.\nUsage: devtimer start <description>");
                return;
            }

            let text_input = args[2..].join(" ");
            let new_text = SavedTime { text: text_input.clone(), timestamp_start: datetime.clone(), timestamp_stop: None };

            let file_path = "saved_times.json";
            let mut texts: Vec<SavedTime> = if let Ok(data) = fs::read_to_string(file_path) {
                serde_json::from_str(&data).unwrap_or(Vec::new())
            } else {
                Vec::new()
            };

            texts.push(new_text);

            let json_data = serde_json::to_string_pretty(&texts).unwrap();
            fs::write(file_path, json_data).unwrap();

            println!("Timer started: {}", args[2..].join(" "));
            
        }
        "stop" => {
            println!("Timer stopped.");

            let file_path = "saved_times.json";

            let mut texts: Vec<SavedTime> = if let Ok(data) = fs::read_to_string(file_path) {
                serde_json::from_str(&data).unwrap_or(Vec::new())
            } else {
                Vec::new()
            };

            if let Some(last) = texts.last_mut() {
                last.timestamp_stop = Some(datetime.clone());
            } else {
                println!("Error: No running timer found to stop.");
                return;
            }

            let json_data = serde_json::to_string_pretty(&texts).unwrap();
            fs::write(file_path, json_data).unwrap();
        }
        "statistics" => {

            let file_path = "saved_times.json";
            if let Ok(data) = fs::read_to_string(file_path) {
                let texts: Vec<SavedTime> = serde_json::from_str(&data).unwrap_or(Vec::new());
                let last_entry = texts.last().unwrap();
                let time_difference = if let Some(stop) = &last_entry.timestamp_stop {
                    let start_time = chrono::NaiveDateTime::parse_from_str(&last_entry.timestamp_start, "%Y-%m-%d %H:%M:%S").unwrap();
                    let stop_time = chrono::NaiveDateTime::parse_from_str(stop, "%Y-%m-%d %H:%M:%S").unwrap();
                    stop_time - start_time
                } else {
                    println!("Error: Timer is still running. Please stop it first.");
                    return;
                };

                let total_seconds = time_difference.num_seconds();

                let days = total_seconds / 86400; 
                let hours = (total_seconds % 86400) / 3600;
                let minutes = (total_seconds % 3600) / 60;
                let seconds = total_seconds % 60;
                
                println!("Task: {}\nDuration: {} days, {} hours, {} minutes, {} seconds", last_entry.text, days, hours, minutes, seconds);
                
            } else {
                println!("Error: No tracked times found.");
            }
            
        }
        _ => {
            println!("Error: Unknown command '{}'\nRun 'devtimer' without arguments to see available commands.", command);
        }
    }
}

