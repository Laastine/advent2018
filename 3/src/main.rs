use std::{fs::File, io::Read, path::Path};
use std::collections::HashMap;

#[derive(Debug)]
struct Line<'a> {
  pub id: &'a str,
  pub pos: (usize, usize),
  pub size: (usize, usize)
}

impl<'a> Line<'a> {
  pub fn new(id: &'a str, pos: (usize, usize), size: (usize, usize)) -> Self {
    Line {
      id,
      pos,
      size
    }
  }
}

#[derive(Debug)]
struct Area {
  pub area: HashMap<(usize, usize), usize>
}

impl Area {
  pub fn new() -> Self {
    Area {
      area: HashMap::new()
    }
  }

  pub fn add_rectangle(&mut self, pos: (usize, usize), size: (usize, usize)) {
    for x in (pos.0)..(size.0 + pos.0) {
      for y in (pos.1)..(size.1 + pos.1) {
        self.area.entry((x, y))
            .and_modify(|e| { *e = 2 })
            .or_insert(1);
      }
    }
  }

  fn has_only_ones(&mut self, pos: (usize, usize), size: (usize, usize)) -> bool {
    for x in (pos.0)..(size.0 + pos.0) {
      for y in (pos.1)..(size.1 + pos.1) {
        if self.area[&(x, y)] > 1 {
          return false
        }
      }
    }
    true
  }

  pub fn find_rectangle_with_ones<'a>(&mut self, data: &'a Line) -> Option<&'a str> {
    if self.has_only_ones(data.pos, data.size) {
      return Some(data.id)
    }
    None
  }

  pub fn get_areas(&self) -> (usize, usize) {
    self.area.iter()
        .fold((0, 0), |acc, (_, &val)| {
          if val == 1 { (acc.0 + 1, acc.1) } else if val == 2 { (acc.0, acc.1 + 1) } else { acc }
        })
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

fn parse_line(line: &str) -> Line {
  let elems = line.split(' ').collect::<Vec<&str>>();
  let pos = elems[2].trim_matches(':').split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
  let size = elems[3].split('x').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
  Line::new(elems[0], (pos[0], pos[1]), (size[0], size[1]))
}

fn main() {
  let data = read_input_file("./input.txt");
  let mut area = Area::new();
  let lines = lines_to_vec(&data).iter().map(|el| parse_line(*el)).collect::<Vec<Line>>();

  lines.iter().for_each(|el| area.add_rectangle(el.pos, el.size));
  println!("Part one: {:?}", area.get_areas());

  let line = lines.iter()
                  .find(|&el| area.find_rectangle_with_ones(el).is_some()).unwrap();
  println!("Part two: {:?}", line.id);
}


/**
........
...2222.
...2222.
.11XX22.
.11XX22.
.111133.
.111133.
........
*/
#[test]
fn basic_test() {
  let input = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"];
  let mut area = Area::new();
  input.iter()
       .map(|el| parse_line(*el))
       .for_each(|el| area.add_rectangle(el.pos, el.size));
  assert_eq!(area.get_areas(), (28, 4))
}

/**
........
...2222.
...2222.
.11XX22.
.11XX22.
.111133.
.111133.
........
*/
#[test]
fn rectangle_with_ones_test() {
  let input = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"];
  let mut area = Area::new();
  let lines = input.iter()
                   .map(|el| parse_line(*el))
                   .collect::<Vec<Line>>();

  lines.iter().for_each(|el| area.add_rectangle(el.pos, el.size));
  let line = lines.iter()
                  .find(|&el| area.find_rectangle_with_ones(el).is_some()).unwrap();
  assert_eq!(line.id, "#3");
}
