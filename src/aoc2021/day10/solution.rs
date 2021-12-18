use std::collections::LinkedList;

use itertools::Itertools;

fn read_data() -> Vec<String> {
  let relative_path = "/src/aoc2021/day10/input";
  let path = crate::utils::input_path(relative_path);

  let contents = std::fs::read_to_string(path).unwrap();
  
  contents
    .split("\n")
    .map(|s| String::from(s))
    .collect()
}

fn get_score(s: &String) -> i32 {
  let mut stk = LinkedList::new();

  for c in s.chars() {
    if c == '(' || c == '[' || c == '{' || c == '<' {
      stk.push_front(c);
    }

    if c == ')' {
      if !stk.is_empty() && *stk.front().unwrap() == '(' {
        stk.pop_front();
      } else {
        return 3;
      }
    }

    if c == ']' {
      if !stk.is_empty() && *stk.front().unwrap() == '[' {
        stk.pop_front();
      } else {
        return 57;
      }
    }

    if c == '}' {
      if !stk.is_empty() && *stk.front().unwrap() == '{' {
        stk.pop_front();
      } else {
        return 1197;
      }
    }

    if c == '>' {
      if !stk.is_empty() && *stk.front().unwrap() == '<' {
        stk.pop_front();
      } else {
        return 25137;
      }
    }
  }

  0
}

fn calc_total_score(data: &Vec<String>) -> i32 {
  data
    .iter()
    .map(|s| get_score(s))
    .sum()
}

fn calc_middle_score(data: &Vec<String>) -> i128 {
  fn is_incomplete(s: &String) -> bool {
    get_score(s) == 0
  }

  fn completion_score(s: &String) -> i128 {
    let mut score = 0;

    let mut stk = LinkedList::new();
    for c in s.chars() {
      if c == '(' || c == '[' || c == '{' || c == '<' {
        stk.push_front(c);
      } else {
        stk.pop_front();
      }
    }

    while !stk.is_empty() {
      let c = stk.pop_front().unwrap();
      if c == '(' {
        score = score * 5 + 1;
      }
      if c == '[' {
        score = score * 5 + 2;
      }
      if c == '{' {
        score = score * 5 + 3;
      }
      if c == '<' {
        score = score * 5 + 4;
      }
    }

    score
  }

  let incomplete_strings = data
    .iter()
    .filter(|&s| is_incomplete(s))
    .cloned()
    .collect::<Vec<_>>();

  let scores = incomplete_strings
    .iter()
    .map(|s| {
      completion_score(s)
    })
    .sorted()
    .collect::<Vec<i128>>();

  scores[scores.len() / 2]
}

pub fn syntax_scoring() {
  let data = read_data();

  let res1 = calc_total_score(&data);
  let res2 = calc_middle_score(&data);
  println!("the total syntax error score for those errors is {}", res1);
  println!("the middle score is {}", res2);
}

#[cfg(test)]
mod tests {
  #[test]
  fn aoc2021_day10() {
    super::syntax_scoring();
  }
}