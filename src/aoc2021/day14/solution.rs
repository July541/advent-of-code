use std::collections::HashMap;

use itertools::Itertools;

type Rules = HashMap<char, HashMap<char, char>>;

fn read_rules() -> Rules {
  let relative_path = "/src/aoc2021/day14/input";
  let path = crate::utils::input_path(relative_path);

  let contents = std::fs::read_to_string(path).unwrap();

  let mut ret = Rules::new();

  contents
    .split("\n")
    .for_each(|line| {
      let v = line.split(" -> ").collect::<Vec<_>>();
      let c1 = v[0].chars().nth(0).unwrap();
      let c2 = v[0].chars().nth(1).unwrap();
      let c3 = v[1].chars().nth(0).unwrap();
      ret.entry(c1).or_insert(HashMap::new()).entry(c2).or_insert(c3);
    });

  ret
}

fn apply_once(start: &String, rules: &Rules) -> String {
  let chs = start.chars().collect::<Vec<_>>();
  let mut ret = String::new();
  ret.push(chs[0]);

  for i in 1..chs.len() {
    let c1 = chs[i - 1];
    let c2 = chs[i];
    ret.push(rules[&c1][&c2]);
    ret.push(c2)
  }

  ret
}

fn count_str(s: &String) -> HashMap<char, i128> {
  let mut ret = HashMap::new();

  for c in s.chars() {
    *ret.entry(c).or_insert(0) += 1;
  }

  ret
}

fn count_apply_nth(start: &String, rules: &Rules, times: usize) -> i128 {
  let mut cur = start.clone();
  (0..times).for_each(|_| {
    cur = apply_once(&cur, rules);
  });

  let counter = count_str(&cur);

  let max = counter.values().max().unwrap();
  let min = counter.values().min().unwrap();

  max - min
}

fn fast_count_apply_nth(start: &String, rules: &Rules, times: usize) -> i128 {
  let chs = start.chars().collect_vec();
  let mut pairs = HashMap::new();
  let mut alphabet = HashMap::new();

  *alphabet.entry(chs[0]).or_insert(0) += 1;
  for i in 1..chs.len() {
    let mut s = String::new();
    s.push(chs[i - 1]);
    s.push(chs[i]);
    *pairs.entry(s).or_insert(0) += 1;
    *alphabet.entry(chs[i]).or_insert(0) += 1;
  }

  for _ in 0..times {
    let mut cur = HashMap::new();
    for p in &pairs {
      let c1 = p.0.chars().nth(0).unwrap();
      let c2 = p.0.chars().nth(1).unwrap();
      let new = rules[&c1][&c2];
      
      let mut s1 = String::new();
      s1.push(c1);
      s1.push(new);
      *cur.entry(s1).or_insert(0) += *p.1;
      
      let mut s2 = String::new();
      s2.push(new);
      s2.push(c2);
      *cur.entry(s2).or_insert(0) += *p.1;

      *alphabet.entry(new).or_insert(0) += *p.1;
    }
    pairs = cur;
  }

  let iter = alphabet.values().map(|x| x);
  iter.clone().max().unwrap() - iter.min().unwrap()
}

pub fn extended_polymerization() {
  let start = String::from("SNVVKOBFKOPBFFFCPBSF");
  let rules = read_rules();

  let res1 = count_apply_nth(&start, &rules, 10);
  let res2 = fast_count_apply_nth(&start, &rules, 40);
  println!("the result after 10 tiems is {}", res1);
  println!("the result after 40 times is {}", res2);
}

#[cfg(test)]
mod tests {
  #[test]
  fn aoc2021_day14() {
    super::extended_polymerization();
  }
}