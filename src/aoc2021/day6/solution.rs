fn read_fishes() -> Vec<i128> {
  let relative_path = "/src/aoc2021/day6/input";
  let path = crate::utils::input_path(relative_path);

  let contents = std::fs::read_to_string(path).unwrap();
  let fishes = contents
    .split(",")
    .map(|x| x.parse::<i128>().unwrap())
    .collect::<Vec<i128>>();

  fishes
}

fn count_fish(fishes: &mut Vec<i128>, day: i32) -> usize {
  const NEW_FISH_DAY: i128 = 8;
  
  fn one_day(fishes: &mut Vec<i128>) {
    fishes.iter_mut().for_each(|x| *x -= 1);
    let new_fish_cnt = fishes.iter().filter(|x| **x == -1).count();
    fishes.iter_mut().for_each(|x| *x = if *x < 0 { 6 } else { *x });
    let mut new_fishs = vec![NEW_FISH_DAY; new_fish_cnt];
    fishes.append(&mut new_fishs);
  }

  for _ in 0..day {
    one_day(fishes);
  }

  fishes.len()
}

fn count_fish_256(fishes: &Vec<i128>) -> i128 {
  let v = (0..7).map(|x| count_fish(&mut vec![x], 256) as i128).collect::<Vec<_>>();
  fishes.iter().map(|x| v[*x as usize]).sum()
}

pub fn lantern_fish() {
  let fishes = &mut read_fishes();

  let res1 = count_fish(fishes, 80);
  let res2 = count_fish_256(fishes);
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