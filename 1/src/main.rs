use std::{fs::File, io::Read, path::Path};
use std::hash::Hash;
use std::collections::HashSet;

pub struct CircularBuffer {
  pub data: Vec<isize>,
  pub read_idx: usize,
}

impl CircularBuffer {
  pub fn new(data: Vec<isize>) -> Self {
    CircularBuffer {
      data,
      read_idx: 0,
    }
  }

  pub fn read(&mut self) -> Option<isize> {
    let length = self.data.len();
    if length == 0 {
      return None;
    }
    let el = self.data[self.read_idx];
    if self.read_idx == length - 1 {
      self.read_idx = 0;
    } else {
      self.read_idx += 1;
    }
    Some(el)
  }

  pub fn find_first_duplice_sum(&mut self) -> Vec<isize> {
    let mut sums: Vec<isize> = vec![0];
    loop {
      let el = match self.read() {
        Some(val) => val,
        None => return sums
      };
      let sum = sums.iter().last().unwrap() + el;
      sums.push(sum);
      if !has_uniq_elements(&mut sums) {
        return sums;
      }
    }
  }
}

fn read_input_file(filename: &str) -> String {
  let mut file = File::open(&Path::new(filename))
    .unwrap_or_else(|e| panic!("File {} read error: {}", filename, e));
  let mut buf = String::new();
  file.read_to_string(&mut buf).unwrap_or_else(|e| panic!("Buffered read error: {}", e));
  buf
}

fn numbers_to_vec(input: String) -> Vec<isize> {
  input.split('\n')
    .map(|el| el.parse::<isize>().unwrap_or_else(|e| panic!("Number cast error: {}", e))).collect()
}

fn calculate_sum(input: Vec<isize>) -> isize {
  input.iter().fold(0, |acc, el| acc + el)
}

fn has_uniq_elements<T>(iter: T) -> bool
  where T: IntoIterator, T::Item: Eq + Hash {
  let mut uniq = HashSet::new();
  iter.into_iter().all(move |el| uniq.insert(el))
}

fn main() {
  let numbers = numbers_to_vec(read_input_file("./input.txt"));
  let res_a = calculate_sum(numbers);
  println!("Hello {:#?}", res_a);
}

#[test]
fn basic_test() {
  let mut buf = CircularBuffer::new(vec![1, -2, 3, 1, 1, -2]);
  assert_eq!(buf.find_first_duplice_sum(), vec![0, 1, -1, 2, 3, 4, 2])
}

#[test]
fn test_1() {
  let mut buf = CircularBuffer::new(vec![1, -1]);
  assert_eq!(buf.find_first_duplice_sum(), vec![0, 1, 0])
}

#[test]
fn test_2() {
  let mut buf = CircularBuffer::new(vec![3, 3, 4, -2, -4]);
  assert_eq!(buf.find_first_duplice_sum(), vec![0, 3, 6, 10, 8, 4, 7, 10])
}

#[test]
fn test_3() {
  let mut buf = CircularBuffer::new(vec![-6, 4, 8, 5, -6]);
  assert_eq!(buf.find_first_duplice_sum(), vec![0, -6, -2, 6, 11, 5, -1, 3, 11])
}
