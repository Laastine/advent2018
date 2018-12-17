use std::{fs::File, io::Read, path::Path};

const CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

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

fn calc_nth(x: i32, y: i32) -> i32 {
  (y * 1000 + x)
}

fn position_to_grid_tuples(coords: &Vec<(i32, i32)>) -> Vec<(bool, char, i32, i32, usize)> {
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

fn mark_duplicates(new_ones: &[(bool, char, i32, i32, usize)]) -> Vec<(bool, char, i32, i32, usize)> {
  let mut duplicates = vec![];
  let mut idx = 0;
  new_ones.iter().for_each(|(_, x1_sign, x1, y1, _)| {
    if let Some(val) = new_ones.iter()
      .skip(idx)
      .take_while(|(_, _, _, y2, _)| y2 <= y1)
      .find(|(_, x2_sign, x2, y2, _)| y1 == y2 && x1 == x2 && x1_sign != x2_sign) {
      duplicates.push((true, '.', val.2, val.3, val.4));
    }
    idx += 1;
  });
  duplicates
}

fn fill_one_pass(start_coords: Vec<(bool, char, i32, i32, usize)>) -> Vec<(bool, char, i32, i32, usize)> {
  let mut new_ones = vec![];
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
    let a = calc_nth(*a1, *a2);
    let b = calc_nth(*b1, *b2);
    a.cmp(&b)
  });

  let mut duplicates = mark_duplicates(&new_ones);

  filled_ones.append(&mut duplicates);
  filled_ones.append(&mut new_ones);
  filled_ones.sort_by(|(_, _, a1, a2, _), (_, _, b1, b2, _)| {
    let a = calc_nth(*a1, *a2);
    let b = calc_nth(*b1, *b2);
    a.cmp(&b)
  });
  filled_ones.dedup_by(|(_, _, a1, a2, _), (_, _, b1, b2, _)| calc_nth(*a1, *a2) == calc_nth(*b1, *b2));
  filled_ones
}

fn fill_grid(coords: &Vec<(i32, i32)>) -> Vec<(bool, char, i32, i32, usize)> {
  let mut res = position_to_grid_tuples(&coords);
  let mut iter = 0;

  while iter < 160 {
    res = fill_one_pass(res);
    iter += 1;
  }
  res
}

fn find_biggest_area_which_is_not_expanding_anymore(grid: &[(bool, char, i32, i32, usize)]) -> usize {
  let mut letter_counts = vec![];
  for letter in CHARS.chars() {
    letter_counts.push(count_letter(&grid, letter));
  }
  letter_counts.sort();

  let grid_after = fill_one_pass(grid.to_vec());
  let mut letter_counts_after = vec![];
  for letter in CHARS.chars() {
    letter_counts_after.push(count_letter(&grid_after, letter));
  }
  letter_counts_after.sort();

  letter_counts.iter()
    .zip(letter_counts_after.iter())
    .filter(|x| x.0 == x.1)
    .map(|(a, _)| *a).last().expect("Boom")
}

#[allow(dead_code)]
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

fn calc_grid_size(positions: &[(i32, i32)]) -> ((i32, i32), (i32, i32)) {
  let (max_x, _) = positions.iter()
    .max_by(|&(x1, _), &(x2, _)| x1.cmp(x2))
    .unwrap_or_else(|| panic!("Max X find error"));

  let (_, max_y) = positions.iter()
    .max_by(|&(_, y1), &(_, y2)| y1.cmp(y2))
    .unwrap_or_else(|| panic!("Max Y find error"));

  let (min_x, _) = positions.iter()
    .min_by(|&(x1, _), &(x2, _)| x1.cmp(x2))
    .unwrap_or_else(|| panic!("Max Y find error"));

  let (_, min_y) = positions.iter()
    .min_by(|&(_, y1), &(_, y2)| y1.cmp(y2))
    .unwrap_or_else(|| panic!("Max Y find error"));

  ((*min_x, *min_y), (*max_x, *max_y))
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

fn count_distance_sum(orig_positions: &[(i32, i32)], pos: (i32, i32)) -> i32 {
  orig_positions.iter().map(|(x, y)| distance((*x, *y), pos)).sum()
}

fn distance(a: (i32, i32), b: (i32, i32)) -> i32 {
  (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn find_region_with_all_locations(positions: &Vec<(i32, i32)>, distance: i32) -> usize {
  let ((min_x, min_y), (max_x, max_y)) = calc_grid_size(positions);
  let mut area_sizes = vec![];
  for x in min_x..max_x {
    for y in min_y..max_y {
      let dist_sum = count_distance_sum(&positions, (x,y));
      if dist_sum < distance {
        area_sizes.push(dist_sum);
      }
    }
  }
  area_sizes.len()
}

fn main() {
  let data = read_input_file("./input.txt");
  let mut lines = lines_to_vec(&data);
  let positions = line_to_positions(&mut lines);
  let areas = fill_grid(&positions);
  println!("Part one: {:?}", find_biggest_area_which_is_not_expanding_anymore(&areas));

  let res_b = find_region_with_all_locations(&positions, 10_000);
  println!("Part two: {:?}", res_b);
}

#[test]
fn second_test() {
  let mut lines = lines_to_vec("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9");
  let positions = line_to_positions(&mut lines);

  let res = find_region_with_all_locations(&positions, 32);
  assert_eq!(res, 16);
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
  let areas = fill_grid(&positions);
  print_grid(&areas, grid_size.1);
  assert_eq!(find_biggest_area_which_is_not_expanding_anymore(&areas), 17)
}
