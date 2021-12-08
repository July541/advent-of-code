use std::fs;

use crate::utils::input_path;

fn larger_than_count(nums: &Vec<i32>) -> i32 {
  let mut idx = 1;
  let mut ret = 0;

  while idx < nums.len() {
    let prev = &nums[idx - 1];
    let cur = &nums[idx];

    if cur > prev {
      ret += 1;
    }
    idx += 1
  }

  return ret
}

fn merge_window(nums: &Vec<i32>) -> Vec<i32> {
  let mut ret = Vec::new();
  let mut idx = 2;

  while idx < nums.len() {
    let past_two = &nums[idx - 2];
    let past = &nums[idx - 1];
    let cur = &nums[idx];

    ret.push(past_two + past + cur);
    idx += 1
  }

  return ret;
}

pub fn sonar_sweep() {
  let relative_path = "/src/aoc2021/day1/input";
  let path = input_path(relative_path);

  let contents = fs::read_to_string(path).unwrap();
  let lines: Vec<&str> = contents.split("\n").collect();
  let nums: Vec<i32> = lines.iter().map(|&x| x.parse().unwrap()).collect();

  // Part 1
  let res1 = larger_than_count(&nums);
  println!("{} measurements are larger than the previous measurement", res1);

  // Part 2
  let merged_nums = merge_window(&nums);
  let res2 = larger_than_count(&merged_nums);
  println!("{} sums are larger than the previous sum", res2);
}

#[cfg(test)]
mod tests {
  use super::sonar_sweep;
  #[test]
  fn aoc2021_day1() {
    sonar_sweep();
  }
}