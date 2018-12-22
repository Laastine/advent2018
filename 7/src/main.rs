use std::collections::HashMap;
use std::{fs::File, io::Read, path::Path};
use std::collections::HashSet;

#[derive(Debug)]
struct Rules {
  before: HashSet<char>,
  after: HashSet<char>,
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
    .filter(|e| !e.is_empty())
    .collect::<Vec<&str>>()
}

fn parse_input_line(line: &str) -> Vec<char> {
  line.split_whitespace()
    .filter(|&el| el.chars().count() == 1 && el.chars().all(|e| e.is_uppercase()))
    .flat_map(|el| el.to_string().chars().collect::<Vec<char>>())
    .collect::<Vec<char>>()
}

fn construct_rule_map(characters: &[Vec<char>]) -> HashMap<char, Rules> {
  let mut character_map = HashMap::new();

  characters.iter().for_each(|letters| {
    let mut before = HashSet::new();
    let mut after = HashSet::new();
    before.insert(letters[0]);
    after.insert(letters[1]);
    character_map.entry(letters[0])
      .and_modify(|set: &mut Rules| { set.after.insert(letters[1]); })
      .or_insert(Rules { before: HashSet::new(), after });
    character_map.entry(letters[1])
      .and_modify(|set| { set.before.insert(letters[0]); })
      .or_insert(Rules { before, after: HashSet::new() });
  });

  character_map
}

fn find_first_candidates(rules: &mut HashMap<char, Rules>) -> Vec<char> {
  let mut candidates = rules.iter()
    .filter(|(_, val)| val.before.is_empty())
    .map(|(tasks, _)| *tasks)
    .collect::<Vec<_>>();
  candidates.sort();
  candidates
}

fn iterate_next_candidate(rules: &mut HashMap<char, Rules>, candidate: char) -> Vec<char> {
  let next_candidates = rules.get(&candidate).unwrap().after.iter().cloned().collect::<HashSet<char>>();
  next_candidates.iter()
    .filter(|&key| {
      let rules: &mut Rules = rules.get_mut(&key).unwrap();
      rules.before.remove(&candidate);
      rules.before.is_empty()
    })
    .cloned()
    .collect::<Vec<_>>()
}

fn sort_characters(parsed_data: &[Vec<char>]) -> String {
  let mut character_mapping = construct_rule_map(&parsed_data);
  let mut sorted_candidates = find_first_candidates(&mut character_mapping);
  let mut result = vec![];

  while !sorted_candidates.is_empty() {
    let candidate = sorted_candidates.remove(0);
    result.push(candidate);
    iterate_next_candidate(&mut character_mapping, candidate).iter()
      .for_each(|&new_candidate| {
        sorted_candidates.push(new_candidate);
      });
    sorted_candidates.sort();
  }
  result.iter().cloned().collect::<String>()
}

fn main() {
  let data = read_input_file("./input.txt");
  let lines = lines_to_vec(&data);

  let parsed_data = lines.iter()
    .map(|&line| parse_input_line(line))
    .collect::<Vec<_>>();

  println!("Part one {}", sort_characters(&parsed_data));
}

#[allow(dead_code)]
fn test_shorthand(lines: Vec<&str>) -> String {
  let parsed_data = lines.iter()
    .map(|&line| parse_input_line(line))
    .collect::<Vec<_>>();

  sort_characters(&parsed_data)
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

  assert_eq!(&test_shorthand(lines), "CABDFE");
}

#[test]
fn second_test() {
  let lines = vec![
    "Step F must be finished before step E can begin.",
    "Step E must be finished before step D can begin.",
    "Step D must be finished before step C can begin.",
    "Step C must be finished before step B can begin.",
    "Step B must be finished before step A can begin.",
    "Step K must be finished before step A can begin.",
    "Step L must be finished before step A can begin.",
  ];

  assert_eq!(&test_shorthand(lines), "FEDCBKLA");
}

#[test]
fn third_test() {
  let lines = vec![
    "Step A must be finished before step O can begin.",
    "Step F must be finished before step O can begin.",
    "Step E must be finished before step O can begin.",
    "Step D must be finished before step B can begin.",
    "Step C must be finished before step O can begin.",
    "Step B must be finished before step O can begin.",
    "Step K must be finished before step O can begin.",
    "Step L must be finished before step O can begin.",
  ];

  assert_eq!(&test_shorthand(lines), "ACDBEFKLO");
}
