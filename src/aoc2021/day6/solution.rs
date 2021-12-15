use std::collections::HashMap;

fn read_fishes() -> Vec<i32> {
  let relative_path = "/src/aoc2021/day6/input";
  let path = crate::utils::input_path(relative_path);

  let contents = std::fs::read_to_string(path).unwrap();
  let fishes = contents
    .split(",")
    .map(|x| x.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

  fishes
}

fn count_fish(fishes: &Vec<i32>, day: i32) -> u128 {
  let mut mp:HashMap<i32, u128> = std::collections::HashMap::new();

  fishes.iter().for_each(|x| {
    *mp.entry(*x).or_insert(0) += 1;
  });
  
  for _ in 0..day {
    let mut cur = HashMap::new();
    for (k, v) in mp {
      if k == 0 {
        *cur.entry(6).or_insert(0) += v;
        *cur.entry(8).or_insert(0) += v;
      } else {
        *cur.entry(k - 1).or_insert(0) += v;
      }
    }
    mp = cur;
  }

  mp.into_iter().map(|x| x.1).sum()
}

pub fn lantern_fish() {
  let fishes = read_fishes();

  let res1 = count_fish(&fishes, 80);
  let res2 = count_fish(&fishes, 256);
  println!("{} lanternfish would be there after 80 days.", res1);
  println!("{} lanternfish would be there after 256 days.", res2);
}

#[cfg(test)]
mod tests {
  #[test]
  fn aoc2021_day6() {
    super::lantern_fish();
  }
}