extern crate chrono;

use chrono::prelude::*;
use std::{fs::File, io::Read, path::Path};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Entry<'a> {
  date_time: chrono::DateTime<Utc>,
  action: &'a str,
}

impl<'a> Entry<'a> {
  pub fn new(year: i32, month: u32, day: u32, hour: u32, minute: u32, action: &'a str) -> Self {
    Entry {
      date_time: Utc.ymd(year, month, day).and_hms(hour, minute, 0),
      action,
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

  pub fn process_sleep_times(&mut self, log: &'a [Entry]) {
    let mut curr_quard = "";
    let mut is_awake = true;
    let mut bed_time = Utc::now();
    log.iter().for_each(|entry| {
      let new_guard = parse_guard(entry.action);
      let is_awake_sleep_line = new_guard.is_empty();
      if is_awake_sleep_line {
        is_awake = parse_awake_info(entry.action);
        if !is_awake {
          bed_time = entry.date_time;
        } else {
          let sleep_time = entry.date_time.minute() - bed_time.minute();
          self.sleeps.entry(&curr_quard)
            .and_modify(|e| { *e += sleep_time })
            .or_insert(sleep_time);
        }
      } else {
        curr_quard = new_guard
      }
    })
  }

  pub fn get_sleepiest_elf(&self) -> (&str, u32) {
    let mut biggest = ("", 0);
    for (key, &val) in self.sleeps.iter() {
      if val > biggest.1 {
        biggest = (key, val)
      }
    }
    biggest
  }

  pub fn most_common_minute_to_sleep(&self, elf_id: &str, log: &[Entry]) -> (u32, u32) {
    let mut sleep_times = vec![];
    let mut curr_quard = "";
    let mut is_awake = true;
    let mut bed_time = Utc::now();
    log.iter().for_each(|entry| {
      let new_guard = parse_guard(entry.action);
      let is_awake_sleep_line = new_guard.is_empty();
      if curr_quard == elf_id && is_awake_sleep_line {
        is_awake = parse_awake_info(entry.action);
        if !is_awake {
          bed_time = entry.date_time;
        } else {
          let mut times = calc_minute_range(&bed_time, &entry.date_time);
          sleep_times.append(&mut times);
        }
      } else {
        curr_quard = new_guard
      }
    });
    let mut minutes: HashMap<u32, u32> = HashMap::new();
    sleep_times.iter().for_each(|el| {
      minutes.entry(*el)
        .and_modify(|x| *x += 1)
        .or_insert(1);
    });
    match minutes.iter().max_by(|(_, &a), (_, &b)| a.cmp(&b)) {
      Some(val) => (*val.0, *val.1),
      None => panic!("No value found")
    }
  }

  pub fn most_minute_to_sleep_by_one_elf(&self, log: &[Entry]) -> (&str, u32, u32) {
    let all_ids = self.sleeps.iter().map(|(&id, _), | id).collect::<Vec<_>>();
    all_ids.iter().map(|&id| {
      let times = self.most_common_minute_to_sleep(id, log);
      (id, times.0, times.1)
    })
      .max_by(|(_, _, a), (_, _, b)| a.cmp(&b))
      .unwrap_or(("", 0, 0))
  }
}

fn calc_minute_range(a: &DateTime<Utc>, b: &DateTime<Utc>) -> Vec<u32> {
  let mut sleep_times = vec![];

  for min in a.minute()..b.minute() {
    sleep_times.push(min);
  }
  sleep_times
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
  entry.starts_with("wakes up")
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
    .map(|el| el.parse::<u32>().unwrap_or_else(|_| panic!("Number cast error")))
    .collect::<Vec<u32>>();

  Entry::new(date[0] as i32,
             date[1],
             date[2],
             date[3], date[4],
             date_time_and_action[1])
}

fn multiply_id_with_minute(id: &str, minute: u32) -> u32 {
  let num = id
    .trim_start_matches('#')
    .parse::<u32>()
    .unwrap_or_else(|e| panic!("Number cast error {:?}", e));
  minute * num
}

fn main() {
  let data = read_input_file("./input.txt");
  let lines = lines_to_vec(&data);
  let mut parsed_lines = lines.iter()
    .map(|&el| parse_line(el))
    .collect::<Vec<Entry>>();

  parsed_lines.sort_by(|a, b| {
    let duration_a = a.date_time.timestamp();
    let duration_b = b.date_time.timestamp();
    duration_a.cmp(&duration_b)
  });

  let mut sleep_calculator = SleepTime::new();
  sleep_calculator.process_sleep_times(&parsed_lines);
  let sleepiest_and_sleeps = sleep_calculator.get_sleepiest_elf();
  let (most_common_minute, _) = sleep_calculator.most_common_minute_to_sleep(sleepiest_and_sleeps.0, &parsed_lines);

  println!("Part one: {} & {} => {}", sleepiest_and_sleeps.0, most_common_minute, multiply_id_with_minute(sleepiest_and_sleeps.0, most_common_minute));

  let res = sleep_calculator.most_minute_to_sleep_by_one_elf(&parsed_lines);
  println!("Part two: {} & {} => {}", res.0, res.1, multiply_id_with_minute(res.0, res.1));
}

#[test]
fn first_test() {
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
    let duration_a = a.date_time.timestamp();
    let duration_b = b.date_time.timestamp();
    duration_a.cmp(&duration_b)
  });

  let mut sleep_calculator = SleepTime::new();
  sleep_calculator.process_sleep_times(&parsed_lines);

  let sleepiest = sleep_calculator.get_sleepiest_elf();

  assert_eq!(sleepiest.0, "#10");
  assert_eq!(sleepiest.1, 50);
  let times = sleep_calculator.most_common_minute_to_sleep("#10", &parsed_lines);
  assert_eq!(times.0, 24);
}

#[test]
fn second_test() {
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
    let duration_a = a.date_time.timestamp();
    let duration_b = b.date_time.timestamp();
    duration_a.cmp(&duration_b)
  });

  let mut sleep_calculator = SleepTime::new();
  sleep_calculator.process_sleep_times(&parsed_lines);

  let res = sleep_calculator.most_minute_to_sleep_by_one_elf(&parsed_lines);
  assert_eq!(res.0, "#99");
  assert_eq!(res.1, 45);
}
