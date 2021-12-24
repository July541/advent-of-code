use std::{collections::HashSet, cmp::Reverse};
use std::panic;

use itertools::Itertools;
use priority_queue::PriorityQueue;

type Board = Vec<Vec<i32>>;

fn read_data() -> Board {
  let relative_path = "/src/aoc2021/day15/input";
  let path = crate::utils::input_path(relative_path);

  let contents = std::fs::read_to_string(path).unwrap();

  contents
    .split("\n")
    .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect_vec())
    .collect_vec()
}

fn calc_risk(mp: &Board) -> i32 {
  let mut que = PriorityQueue::new();
  let mut st = HashSet::new();
  que.push((0, 0, 0), Reverse((0, 0, 0)));
  st.insert((0, 0));
  let endx = mp.len() as i32 - 1;
  let endy = mp.last().unwrap().len() as i32 - 1;

  let dir = [(-1, 0), (0, 1), (1, 0), (0, -1)];

  while !que.is_empty() {
    let q = que.pop().unwrap().0;

    if q.1 == endx && q.2 == endy {
      return q.0;
    }

    for d in &dir {
      let x = q.1 + d.0;
      let y = q.2 + d.1;
      if x >= 0 && x <= endx && y >= 0 && y <= endy {
        if !st.contains(&(x, y)) {
          st.insert((x, y));
          let item = (mp[x as usize][y as usize] + q.0, x, y);
          que.push(item, Reverse(item));
        }
      }
    }
  }

  panic!("No result");
}

fn calc_risk_for_five_times(mp: &Board) -> i32 {
  let row = mp.len();
  let col = mp[0].len();

  let mut new = Vec::new();
  for i in 0..row * 5 {
    new.push(Vec::new());
    for _ in 0..col * 5 {
      new[i].push(0);
    }
  }

  fn wrapper(x: i32) -> i32 {
    (x - 1) % 9 + 1
  }

  for i in 0..row * 5 {
    for j in 0..col * 5 {
      new[i][j] = wrapper(mp[i % row][j % col] + ((i / row) as i32) + ((j / col) as i32));
    }
  }

  calc_risk(&new)
}

pub fn chiton() {
  let mp = read_data();

  let res1 = calc_risk(&mp);
  let res2 = calc_risk_for_five_times(&mp);
  println!("the risk is {}", res1);
  println!("the risk is {}", res2);
}

#[cfg(test)]
mod tests {
  #[test]
  fn aoc2021_day15() {
    super::chiton();
  }
}