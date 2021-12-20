use std::collections::HashSet;

fn read_data() -> Vec<Vec<i32>> {
  let relative_path = "/src/aoc2021/day11/input";
  let path = crate::utils::input_path(relative_path);

  let contents = std::fs::read_to_string(path).unwrap();

  contents
    .split("\n")
    .map(|line| line
      .chars()
      .map(|c| c.to_digit(10).unwrap() as i32)
      .collect())
    .collect()
}

fn calc_times(energies: &Vec<Vec<i32>>) -> i32 {
  let mut total_flash_times = 0;

  fn one_step(v: &mut Vec<Vec<i32>>) -> i32 {
    let mut flash_times = 0;

    let dir = vec![
      vec![-1, -1]
    , vec![-1, 0]
    , vec![-1, 1]
    , vec![0, -1]
    , vec![0, 1]
    , vec![1, -1]
    , vec![1, 0]
    , vec![1, 1]
    ];

    v
      .iter_mut()
      .for_each(|row| row
        .iter_mut()
        .for_each(|x| *x += 1));

    let mut flash_pos:HashSet<(usize, usize)> = HashSet::new();

    loop {
      let cnt = flash_pos.len();

      for i in 0..v.len() {
        for j in 0..v[i].len() {
          let t = v[i][j];
          if t >= 10 && !flash_pos.contains(&(i, j)) {
            flash_pos.insert((i, j));
            for d in &dir {
              let x = i as i32 + d[0];
              let y = j as i32 + d[1];
  
              if x >= 0 && x < v.len() as i32 && y >= 0 && y < v[x as usize].len() as i32 && v[x as usize][y as usize] < 10 {
                v[x as usize][y as usize] += 1;
              }
            }
          }
        }
      }

      if cnt == flash_pos.len() {
        break;
      }
    }

    v
      .iter_mut()
      .for_each(|row| row
        .iter_mut()
        .for_each(|x| {
          if *x >= 10 {
            *x = 0;
            flash_times += 1;
          }
        }));

    flash_times
  }

  fn all_flash(data: &Vec<Vec<i32>>) -> bool {
    data
      .iter()
      .all(|row| row.iter().all(|x| *x == 0))
  }

  let mut data = energies.clone();
  let mut found = false;
  (1..=100).for_each(|t| {
    total_flash_times += one_step(&mut data);
    if !found && all_flash(&data) {
      found = true;
      println!("all octopuses flash at {}", t);
    }
  });
  (101..=1000).for_each(|t| {
    one_step(&mut data);
    if !found && all_flash(&data) {
      found = true;
      println!("all octopuses flash at {}", t);
    }
  });
  total_flash_times
}

pub fn dumbo_octopus() {
  let energies = read_data();

  let res1 = calc_times(&energies);
  println!("flash {} times", res1);
}

#[cfg(test)]
mod tests {
  #[test]
  fn aoc2021_day11() {
    super::dumbo_octopus();
  }
}