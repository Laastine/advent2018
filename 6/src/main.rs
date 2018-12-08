use std::{fs::File, io::Read, path::Path};

const CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
//const CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

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

fn line_to_chars(input: &str) -> Vec<char> {
  input.chars()
    .filter(|chr| chr != &'\n')
    .collect::<Vec<char>>()
}

fn line_to_positions(lines: &mut Vec<&str>) -> Vec<(i32, i32)> {
  lines.iter()
    .map(|&line| {
      let nums = line.split(", ")
        .map(|x| x.parse::<i32>().unwrap_or_else(|_| panic!("Number cast error {}", x)))
        .collect::<Vec<i32>>();
      (nums[0], nums[1])
    })
    .collect::<Vec<(i32, i32)>>()
}

fn fill_neighbourhood(coord: (char, i32, i32, usize)) -> Vec<(bool, char, i32, i32, usize)> {
  let sign = coord.0;
  let nth_pass = coord.3 + 1;
  vec![
    (false, sign, (coord.1), (coord.2 - 1), nth_pass),
    (false, sign, (coord.1 - 1), (coord.2), nth_pass),
    (false, sign, (coord.1), (coord.2 + 1), nth_pass),
    (false, sign, (coord.1 + 1), (coord.2), nth_pass)
  ]
}

fn calc_nth(x: &i32, y: &i32) -> i32 {
  (y * 1000 + x)
}

fn position_to_grid_tuples(coords: Vec<(i32, i32)>) -> Vec<(bool, char, i32, i32, usize)> {
  let mut sign = ' ';
  coords.iter()
    .enumerate()
    .map(|(idx, &coord)| {
       sign = *line_to_chars(CHARS)
        .get(idx)
        .unwrap_or_else(|| panic!("Indexing error {}", idx));
      (true,
       sign,
       coord.0,
       coord.1,
       0,
      )
    })
    .collect::<Vec<_>>()
}

fn mark_duplicates(new_ones: &Vec<(bool, char, i32, i32, usize)>) -> Vec<(bool, char, i32, i32, usize)> {
  let mut duplicates = vec![];
  new_ones.iter().for_each(|(_, x1_sign, x1, y1, _)| {
    if let Some(val) = new_ones.iter().find(|(_, x2_sign, x2, y2, _)| x1_sign != x2_sign && x1 == x2 && y1 == y2) {
      duplicates.push((true, '.', val.2, val.3, val.4));
    }
  });
  duplicates
}

fn new_ones_with_duplicates(new_ones: &mut Vec<(bool, char, i32, i32, usize)>, duplicates: &[(bool, char, i32, i32, usize)]) {
  let mut idx = 0;
  while idx != new_ones.len() {
    let (_, _, x1, y1, _) = new_ones[idx];
    if duplicates.iter().any(|(_, _, x2, y2, _)| (x1 == *x2 && y1 == *y2)) {
      new_ones.remove(idx);
    } else {
      idx += 1;
    }
  }
}

fn fill_one_pass(start_coords: Vec<(bool, char, i32, i32, usize)>) -> Vec<(bool, char, i32, i32, usize)> {
  let mut new_ones= vec![];
  for el in start_coords.iter().filter(|&el| el.0) {
    let nth_pass = el.4 + 1;
    let sign = el.1;
    new_ones.push((false, sign, (el.2), (el.3 - 1), nth_pass));
    new_ones.push((false, sign, (el.2 - 1), (el.3), nth_pass));
    new_ones.push((false, sign, (el.2), (el.3 + 1), nth_pass));
    new_ones.push((false, sign, (el.2 + 1), (el.3), nth_pass));
  }

  let mut filled_ones = start_coords.iter()
    .map(|pos| (true, pos.1, pos.2, pos.3, pos.4))
    .collect::<Vec<_>>();

  new_ones.sort_by(|(_, _, a1, a2, _), (_, _, b1, b2, _)| {
    let a = calc_nth(&a1, &a2);
    let b = calc_nth(&b1, &b2);
    a.cmp(&b)
  });
  filled_ones.dedup_by(|(_, _, a1, a2, _), (_, _, b1, b2, _)| calc_nth(&a1, &a2) == calc_nth(&b1, &b2));

  let mut duplicates = mark_duplicates(&new_ones);

  filled_ones.append(&mut duplicates);
//  new_ones_with_duplicates(&mut new_ones, &duplicates);
  filled_ones.append(&mut new_ones);
  filled_ones.sort_by(|(_, _, a1, a2, _), (_, _, b1, b2, _)| {
    let a = calc_nth(&a1, &a2);
    let b = calc_nth(&b1, &b2);
    a.cmp(&b)
  });
  filled_ones.dedup_by(|(_, _, a1, a2, _), (_, _, b1, b2, _)| calc_nth(&a1, &a2) == calc_nth(&b1, &b2));
  filled_ones
}

fn fill_grid(coords: Vec<(i32, i32)>) -> Vec<(bool, char, i32, i32, usize)> {
  let start_coords = position_to_grid_tuples(coords);

  let mut res = start_coords;
  let mut iter = 0;
  while iter < 10 {
    res = fill_one_pass(res);
    iter += 1;
    println!("{}", iter);
  }
  res
}

fn print_grid(grid: &[(bool, char, i32, i32, usize)], grid_size: (i32, i32)) {
  let extra = 1;
  let default = (false, '_', 0, 0, 0);
  for y in 0..(grid_size.1 + extra) {
    for x in 0..=(grid_size.0 + extra) {
      let (_, letter, _, _, _) = grid.iter()
        .find(|(_, _, gx, gy, _)| *gx == x && *gy == y)
        .unwrap_or(&default);
      print!("{}", &letter);
    }
    println!();
  }
}

fn calc_grid_size(positions: &[(i32, i32)]) -> (i32, i32) {
  let (max_x, _) = positions.iter()
    .max_by(|&(x1, _), &(x2, _)| x1.cmp(x2))
    .unwrap_or_else(|| panic!("Max X find error"));

  let (_, max_y) = positions.iter()
    .max_by(|&(_, y1), &(_, y2)| y1.cmp(y2))
    .unwrap_or_else(|| panic!("Max Y find error"));

  (*max_x, *max_y)
}

fn count_letter(grid: &[(bool, char, i32, i32, usize)], letter: char) -> usize {
  let mut count = 0;
  for &(_, sign, _, _, _) in grid.iter() {
    if sign == letter {
      count += 1;
    }
  }
  count
}

fn main() {
  let data = read_input_file("./input.txt");
  let mut lines = lines_to_vec(&data);
  let positions = line_to_positions(&mut lines);
  let areas = fill_grid(positions);

  let mut letter_counts = vec![];
  for letter in CHARS.chars() {
    letter_counts.push(count_letter(&areas, letter));
  }
  letter_counts.sort();
  println!("Part one: {:?}", letter_counts);
}

/**
aaaaa.cccc
aAaaa.cccc
aaaddecccc
aadddeccCc
..dDdeeccc
bb.deEeecc
bBb.eeee..
bbb.eeefff
bbb.eeffff
bbb.ffffFf
*/
#[test]
fn basic_test() {
  let mut lines = lines_to_vec("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9");
  let positions = line_to_positions(&mut lines);
  let grid_size = calc_grid_size(&positions);
  let areas = fill_grid(positions);
  print_grid(&areas, grid_size);
  assert_eq!(count_letter(&areas, 'E'), 17)
}
