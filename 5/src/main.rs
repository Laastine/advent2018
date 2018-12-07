use std::{fs::File, io::Read, path::Path};

const CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn read_input_file(filename: &str) -> String {
  let mut file = File::open(&Path::new(filename))
    .unwrap_or_else(|e| panic!("File {} read error: {}", filename, e));
  let mut buf = String::new();
  file.read_to_string(&mut buf).unwrap_or_else(|e| panic!("Buffered read error: {}", e));
  buf
}

fn line_to_chars(input: &str) -> Vec<String> {
  input.chars()
    .filter(|chr| chr != &'\n')
    .map(|el| el.to_string())
    .collect::<Vec<String>>()
}

fn is_uppercase(letter: &str) -> bool {
  CHARS.contains(letter)
}

fn is_reacting(a: &str, b: &str) -> bool {
  ((is_uppercase(a) && !is_uppercase(b)) ||
    (!is_uppercase(a) && is_uppercase(b))) &&
    a.to_ascii_lowercase() == b.to_ascii_lowercase()
}

fn parse_one_pass(input: &mut Vec<String>) -> Vec<String> {
  let len = &(input.len() - 1);
  let mut idx_to_be_removed = vec![];
  for (idx, chr) in input.iter().enumerate() {
    if
      idx < *len &&
      is_reacting(chr, input[idx + 1].as_str()) &&
      !idx_to_be_removed.contains(&(idx - 1)) {
      idx_to_be_removed.push(idx);
      idx_to_be_removed.push(idx + 1);
    }
  }
  input.iter_mut().enumerate()
    .map(|(idx, chr)|
      if idx_to_be_removed.contains(&idx) {
        "".to_string()
      } else {
        chr.to_string()
      })
    .filter(|x| !x.is_empty())
    .collect::<Vec<String>>()
}

fn remove_letter(input: &mut Vec<String>, letter: &str) -> Vec<String> {
  input.iter()
    .filter(|&chr| chr.to_ascii_uppercase() != letter)
    .map(|x| x.to_string())
    .collect::<Vec<String>>()
}

fn parse_input_by_letter(input: &mut Vec<String>, letter: &str) -> String {
  let without_letter = remove_letter(input, letter);
  parse_input(without_letter)
}

fn find_problematic_letter(input: &mut Vec<String>) -> Vec<(String, usize)> {
  let mut char_lens = line_to_chars(CHARS).iter()
    .map(|chr| (chr.to_string(), parse_input_by_letter(input, chr).len()))
    .collect::<Vec<(String, usize)>>();
  char_lens.sort_by(|(_, a), (_, b)| a.cmp(b));
  char_lens
}

fn parse_input(mut input: Vec<String>) -> String {
  let mut len_a = input.len();
  let mut len_b = 0;
  while len_a > len_b {
    len_a = input.len();
    input = parse_one_pass(&mut input);
    len_b = input.len();
  }
  input.iter().map(|el| el.as_str()).collect::<String>()
}

fn main() {
  let data = read_input_file("./input.txt");
  let mut chars = line_to_chars(&data);
  let res_a = parse_input(chars.clone());
  println!("Part one: {}", res_a.chars().count());
  let res_b = find_problematic_letter(&mut chars);
  println!("Part two: {:?}", res_b[0]);
}

#[test]
fn one_pass_test() {
  let input = "dabAcCaCBAcCcaDA";
  let mut chars = line_to_chars(input);
  let res = parse_one_pass(&mut chars).into_iter().collect::<String>();
  assert_eq!(res, "dabAaCBAcaDA".to_string());
}

#[test]
fn duplicate_test() {
  let input = "aaaaaaaaaAAA";
  let chars = line_to_chars(input);
  let res = parse_input(chars);
  assert_eq!(res, "aaaaaa".to_string());
}

#[test]
fn basic_test() {
  let input = "dabAcCaCBAcCcaDA";
  let chars = line_to_chars(input);
  let res = parse_input(chars);
  assert_eq!(res, "dabCBAcaDA".to_string());
  assert_eq!(res.chars().count(), 10)
}

#[test]
fn second_part_a() {
  let input = "dabAcCaCBAcCcaDA";
  let mut chars = line_to_chars(input);
  let without_a = remove_letter(&mut chars, "A").into_iter().collect::<String>();
  assert_eq!(without_a, "dbcCCBcCcD");
  let res = parse_input_by_letter(&mut chars, "A");
  assert_eq!(res, "dbCBcD");
}

#[test]
fn second_part_b() {
  let input = "dabAcCaCBAcCcaDA";
  let mut chars = line_to_chars(input);
  let without_b = remove_letter(&mut chars, "B").into_iter().collect::<String>();
  assert_eq!(without_b, "daAcCaCAcCcaDA");

  let res = parse_input_by_letter(&mut chars, "B");
  assert_eq!(res, "daCAcaDA");
}

#[test]
fn second_part_c() {
  let input = "dabAcCaCBAcCcaDA";
  let mut chars = line_to_chars(input);
  let res = parse_input_by_letter(&mut chars, "C");
  assert_eq!(res, "daDA");
}

#[test]
fn second_part_d() {
  let input = "dabAcCaCBAcCcaDA";
  let mut chars = line_to_chars(input);
  let res = parse_input_by_letter(&mut chars, "D");
  assert_eq!(res, "abCBAc");
}

#[test]
fn second_part_combined() {
  let input = "dabAcCaCBAcCcaDA";
  let mut chars = line_to_chars(input);
  let res = find_problematic_letter(&mut chars);
  assert_eq!(res[0].0, "C");
  assert_eq!(res[0].1, 4);
}
