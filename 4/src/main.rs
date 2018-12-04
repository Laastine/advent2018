extern crate chrono;

use chrono::prelude::*;
use std::{fs::File, io::Read, path::Path};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Time {
  pub date_time: chrono::DateTime<Utc>,
}

#[derive(Debug, PartialEq)]
struct Entry<'a> {
  time: Time,
  action: &'a str
}

impl<'a> Entry<'a> {
  pub fn new(year: i32, month: u32, day: u32, hour: u32, minute: u32, action: &'a str) -> Self {
    let time = Time {
      date_time: Utc.ymd(year, month, day).and_hms(hour, minute, 0)
    };
    Entry {
      time,
      action
    }
  }
}

struct SleepTime<'a> {
  pub sleeps: HashMap<&'a str, u32>
}

impl<'a> SleepTime<'a> {
  pub fn new() -> Self {
    SleepTime {
      sleeps: HashMap::new()
    }
  }

  pub fn process_sleep_times(&mut self, log: &'a Vec<Entry>) {
    let mut curr_quard = "";
    let mut is_awake = true;
    let mut bed_time = Duration::now();
    log.iter().for_each(|entry| {
      let guard = parse_guard(entry.action);
      if guard.is_empty() { is_awake = parse_awake_info(entry.action) } else { curr_quard = guard };

      self.sleeps.entry(&curr_quard)
          .and_modify(|e| { *e = 2 })
          .or_insert(1);
    })
  }
}

fn parse_guard(entry: &str) -> &str {
  let id_matches = entry
    .split_whitespace()
    .filter(|el| el.starts_with('#'))
    .collect::<Vec<&str>>();
  if id_matches.is_empty() {
    ""
  } else {
    id_matches[0]
  }
}

fn parse_awake_info(entry: &str) -> bool {
  if entry.starts_with("wakes up") { true } else { false }
}

fn read_input_file(filename: &str) -> String {
  let mut file = File::open(&Path::new(filename))
    .unwrap_or_else(|e| panic!("File {} read error: {}", filename, e));
  let mut buf = String::new();
  file.read_to_string(&mut buf).unwrap_or_else(|e| panic!("Buffered read error: {}", e));
  buf
}

fn lines_to_vec(input: &str) -> Vec<&str> {
  input.lines()
       .collect::<Vec<&str>>()
}

fn parse_line(input: &str) -> Entry {
  let date_time_and_action = input.split("] ")
                                  .collect::<Vec<&str>>();
  let date = date_time_and_action[0]
    .trim_start_matches('[')
    .split(|c| c == '-' || c == ' ' || c == ':')
    .map(|el| el.parse::<u32>().unwrap_or(999999))
    .collect::<Vec<u32>>();

  Entry::new(date[0] as i32,
             date[1],
             date[2],
             date[3], date[4],
             date_time_and_action[1])
}

fn main() {
  let data = read_input_file("./input.txt");
  let lines = lines_to_vec(&data);
  let mut parsed_lines = lines.iter()
                              .map(|&el| parse_line(el))
                              .collect::<Vec<Entry>>();

  parsed_lines.sort_by(|a, b| {
    let duration_a = a.time.date_time.timestamp();
    let duration_b = b.time.date_time.timestamp();
    duration_a.cmp(&duration_b)});

  let formatted = parsed_lines.iter()
                              .map(|el| format!("{:?} {}\n", el.time.date_time, el.action))
                              .collect::<String>();
  println!("Part one {}", formatted);
}

#[test]
fn basic_test() {
  let lines = vec!["[1518-11-01 00:00] Guard #10 begins shift",
                   "[1518-11-01 00:05] falls asleep",
                   "[1518-11-01 00:25] wakes up",
                   "[1518-11-01 00:30] falls asleep",
                   "[1518-11-01 00:55] wakes up",
                   "[1518-11-01 23:58] Guard #99 begins shift",
                   "[1518-11-02 00:40] falls asleep",
                   "[1518-11-02 00:50] wakes up",
                   "[1518-11-03 00:05] Guard #10 begins shift",
                   "[1518-11-03 00:24] falls asleep",
                   "[1518-11-03 00:29] wakes up",
                   "[1518-11-04 00:02] Guard #99 begins shift",
                   "[1518-11-04 00:36] falls asleep",
                   "[1518-11-04 00:46] wakes up",
                   "[1518-11-05 00:03] Guard #99 begins shift",
                   "[1518-11-05 00:45] falls asleep",
                   "[1518-11-05 00:55] wakes up"];

  let mut parsed_lines = lines.iter()
                              .map(|&el| parse_line(el))
                              .collect::<Vec<Entry>>();
  parsed_lines.sort_by(|a, b| {
    let duration_a = a.time.date_time.timestamp();
    let duration_b = b.time.date_time.timestamp();
    duration_a.cmp(&duration_b)});
  let formatted = parsed_lines.iter()
                              .map(|el| format!("{:?} {}\n", el.time.date_time, el.action))
                              .collect::<String>();
  println!("parsed_lines {}", formatted);

  assert_eq!(true, false);
}
