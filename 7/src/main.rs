use std::cmp::Ordering;
use std::{fs::File, io::Read, path::Path};

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

fn parse_input_line(line: &str) -> Vec<char> {
  line.split_whitespace()
    .filter(|&el| el.chars().count() == 1 && el.chars().all(|e| e.is_uppercase()))
    .map(|el| el.to_string().chars().collect::<Vec<char>>())
    .flatten()
    .collect::<Vec<char>>()
}

fn main() {
  let data = read_input_file("./input.txt");
  let mut lines = lines_to_vec(&data);
  println!("{:?}", lines);
}

#[test]
fn first_test() {
  let lines = vec![
    "Step C must be finished before step A can begin.",
    "Step C must be finished before step F can begin.",
    "Step A must be finished before step B can begin.",
    "Step A must be finished before step D can begin.",
    "Step B must be finished before step E can begin.",
    "Step D must be finished before step E can begin.",
    "Step F must be finished before step E can begin.",
  ];

  let mut parsed_data = lines.iter()
    .map(|&line| parse_input_line(line))
    .collect::<Vec<_>>();

  let characters = parsed_data.iter().flatten().collect();

  // TODO: key -> values tietorakenne

  characters.sort_by(|&a,&b| {

  });

  println!("FOO {:?}", parsed_data);

  assert_eq!(true, false);
}
