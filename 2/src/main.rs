use std::{fs::File, io::Read, path::Path};

fn read_input_file(filename: &str) -> String {
  let mut file = File::open(&Path::new(filename))
    .unwrap_or_else(|e| panic!("File {} read error: {}", filename, e));
  let mut buf = String::new();
  file.read_to_string(&mut buf).unwrap_or_else(|e| panic!("Buffered read error: {}", e));
  buf
}

fn lines_to_vec(input: &str) -> Vec<&str> {
  input.split('\n')
    .collect::<Vec<&str>>()
}

fn count_duplicate_elems(input: &str) -> (usize, usize) {
  let orig = input
    .split("")
    .filter(|el| !el.is_empty())
    .collect::<Vec<&str>>();
  let mut uniqs = orig.clone();
  uniqs
    .sort();
  uniqs
    .dedup_by(|a, b| b.eq_ignore_ascii_case(a));

  if uniqs.len() < orig.len() {
    let occurencies = uniqs.iter()
      .map(|character| {
        orig.iter().fold(0, |acc, el| {
          if (*character).eq_ignore_ascii_case(*el) {
            acc + 1
          } else {
            acc
          }
        })
      })
      .collect::<Vec<usize>>();


    uniqs.iter().zip(occurencies.iter())
      .fold((0, 0), |acc, el| {
        if *el.1 == 2 && acc.0 < 1 {
          (acc.0 + 1, acc.1)
        } else if *el.1 == 3 && acc.1 < 1 {
          (acc.0, acc.1 + 1)
        } else {
          acc
        }
      })
  } else {
    (0, 0)
  }
}

fn count_checksum(input: Vec<(usize, usize)>) -> usize {
  let occs = input.iter().fold((0, 0), |acc, el| (acc.0 + el.0, acc.1 + el.1));
  occs.0 * occs.1
}

fn is_one_char_diff(a: &str, b: &str) -> bool {
  let vec_a = a
    .split("")
    .filter(|el| !el.is_empty())
    .collect::<Vec<&str>>();
  let mut vec_b = b
    .split("")
    .filter(|el| !el.is_empty())
    .collect::<Vec<&str>>();

  let matches = vec_a.iter()
    .zip(&mut vec_b)
    .fold(0, |acc, (x, y)| {
      if x.eq_ignore_ascii_case(*y) { acc + 1 } else { acc }
    });

  vec_a.len() == vec_b.len() && matches + 1 == vec_b.len()
}

fn find_one_char_diff<'a>(data: &mut Vec<&'a str>) -> (&'a str, &'a str) {
  let mut res = ("", "");
  for (idx, el) in data.iter().enumerate() {
    data.iter().skip(idx).for_each(|&x| {
      if is_one_char_diff(x, el) {
        res = (x, *el);
      }
    });
  }
  res
}

fn main() {
  let data = read_input_file("./input.txt");
  let mut lines = lines_to_vec(&data);
  let res = lines.iter().map(|el| count_duplicate_elems(*el)).collect::<Vec<_>>();
  let res_a = count_checksum(res);
  println!("Part one: {:#?}", res_a);
  let res_b = find_one_char_diff(&mut lines);
  println!("Part two: {:#?}", res_b);
}

#[test]
fn is_one_char_diff_test() {
  assert!(is_one_char_diff("axabrbcc", "axabsbcc"));
}

#[test]
fn real_diff_test() {
  let mut data = vec!["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"];
  let res = find_one_char_diff(&mut data);
  assert_eq!(res.0, "fguij");
  assert_eq!(res.1, "fghij");
}

#[test]
fn first_row() {
  assert_eq!(count_duplicate_elems("abcdef"), (0, 0));
}

#[test]
fn second_row() {
  assert_eq!(count_duplicate_elems("bababc"), (1, 1));
}

#[test]
fn third_row() {
  assert_eq!(count_duplicate_elems("abbcde"), (1, 0));
}

#[test]
fn fourth_row() {
  assert_eq!(count_duplicate_elems("abcccd"), (0, 1));
}

#[test]
fn fifth_row() {
  assert_eq!(count_duplicate_elems("aabcdd"), (1, 0));
}

#[test]
fn sixth_row() {
  assert_eq!(count_duplicate_elems("abcdee"), (1, 0));
}

#[test]
fn seventh_row() {
  assert_eq!(count_duplicate_elems("ababab"), (0, 1));
}
