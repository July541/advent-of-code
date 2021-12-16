fn read_data() -> Vec<i32> {
  let relative_path = "/src/aoc2021/day7/input";
  let path = crate::utils::input_path(relative_path);

  let contents = std::fs::read_to_string(path).unwrap();

  contents
    .split(",")
    .map(|x| x.parse::<i32>().unwrap())
    .collect()
}

fn calc_fuel_1(positions: &Vec<i32>) -> i32 {
  let max_val = *positions.iter().max().unwrap();

  let mut dp = Vec::new();
  dp.resize(positions.len() + 1, Vec::new());
  dp.iter_mut()
    .for_each(|x| x.resize(max_val as usize + 1, 0));

  for i in 1..=positions.len() {
    dp[i][0] = dp[i - 1][0] + positions[i - 1];
  }

  for i in 1..=positions.len() {
    for j in 1..=max_val as usize {
        dp[i][j] = dp[i - 1][j] + (positions[i - 1] - j as i32).abs();
    }
  }

  *dp[positions.len()].iter().min().unwrap()
}

fn calc_fuel_2(positions: &Vec<i32>) -> i32 {

  fn sum_of_fuel(n: i32) -> i32 {
    (n + 1) * n / 2
  }

  let max_val = *positions.iter().max().unwrap();

  let mut dp = Vec::new();
  dp.resize(positions.len() + 1, Vec::new());
  dp.iter_mut()
    .for_each(|x| x.resize(max_val as usize + 1, 0));

  for i in 1..=positions.len() {
    dp[i][0] = dp[i - 1][0] + sum_of_fuel(positions[i - 1]);
  }

  for i in 1..=positions.len() {
    for j in 1..=max_val as usize {
      dp[i][j] = dp[i - 1][j] + sum_of_fuel((positions[i - 1] - j as i32).abs());
    }
  }

  *dp[positions.len()].iter().min().unwrap()
}

pub fn the_treachery_of_whales() {
  let positions = read_data();

  let res1 = calc_fuel_1(&positions);
  let res2 = calc_fuel_2(&positions);
  println!("spend {} fuel", res1);
  println!("spend {} fuel for new rule", res2);
}

#[cfg(test)]
mod tests {
  #[test]
  fn aoc2021_day7() {
    super::the_treachery_of_whales();
  }
}