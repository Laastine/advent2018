use std::{fs::File, io::Read, path::Path};
use std::collections::HashMap;

struct Area {
  area: HashMap<(usize, usize), usize>
}

impl Area {
  pub fn add_rectangle(&mut self, pos: (usize, usize), size: (usize, usize)) {
    
  }
}

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

fn main() {
  let data = read_input_file("./input.txt");
  println!("{:?}", lines_to_vec(&data));
}
