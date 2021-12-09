use crate::utils::input_path;

fn transpose(mat: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
  let mut ret = Vec::new();
  let mut j = 0;

  while j < mat[0].len() {
    let mut v = Vec::new();
    let mut i = 0;
    while i < mat.len() {
      v.push(mat[i][j]);
      i += 1;
    }
    ret.push(v);
    j += 1;
  }

  return ret;
}

fn calc_consumption(report: &Vec<Vec<u32>>) -> u32 {
  let mat = &transpose(&report);
  let mut gamma_str = String::new();
  let mut epsilon_str = String::new();

  for row in mat {
    let mut ones = 0;
    let mut zeros = 0;
    for digit in row {
      if *digit == 0 as u32 {
        zeros += 1;
      } else {
        ones += 1;
      }
    }

    if ones > zeros {
      gamma_str += "1";
      epsilon_str += "0";
    } else {
      gamma_str += "0";
      epsilon_str += "1";
    }
  }

  let gamma: u32 = isize::from_str_radix(&gamma_str, 2).unwrap().try_into().unwrap();
  let epsilon: u32 = isize::from_str_radix(&epsilon_str, 2).unwrap().try_into().unwrap();

  return gamma * epsilon;
}

fn life_support_rating(mat: &Vec<Vec<u32>>) -> u32 {
  fn filter_values(mat: &Vec<Vec<u32>>, idx: usize, most_common: bool) -> Vec<Vec<u32>> {
    if mat.len() == 1 {
      return mat.clone();
    }

    let mut ones = 0;
    let mut zeros = 0;
    for item in mat {
      if item[idx] == 0 as u32 {
        zeros += 1;
      } else {
        ones += 1;
      }
    }

    let ones_vec = mat
        .iter()
        .filter(|&x| x[idx] == 1 as u32)
        .cloned()
        .collect::<Vec<Vec<u32>>>();
    let zeros_vec = mat
        .iter()
        .filter(|&x| x[idx] == 0 as u32)
        .cloned()
        .collect::<Vec<Vec<u32>>>();

    return if most_common {
      if ones >= zeros {
        filter_values(&ones_vec, idx + 1, most_common)
      } else {
        filter_values(&zeros_vec, idx + 1, most_common)
      }
    } else {
      if ones >= zeros {
        filter_values(&zeros_vec, idx + 1, most_common)
      } else {
        filter_values(&ones_vec, idx + 1, most_common)
      }
    }
  }

  let oxygen_rating_vec_res = filter_values(&mat, 0, true);
  let oxygen_rating_vec = &oxygen_rating_vec_res[0];

  let co2_rating_vec_res = filter_values(&mat, 0, false);
  let co2_rating_vec = &co2_rating_vec_res[0];

  let oxygen_rating_str = oxygen_rating_vec
      .iter()
      .map(|&x| if x == 1 as u32 { '1' } else { '0' })
      .collect::<String>();
  let co2_rating_str = co2_rating_vec
      .iter()
      .map(|&x| if x == 1 as u32 { '1' } else { '0' })
      .collect::<String>();

  let oxygen_rating: u32 = isize::from_str_radix(&oxygen_rating_str, 2).unwrap().try_into().unwrap();
  let co2_rating: u32 = isize::from_str_radix(&co2_rating_str, 2).unwrap().try_into().unwrap();

  return oxygen_rating * co2_rating;
}

pub fn binary_diagnostic() {
  let relative_path = "/src/aoc2021/day3/input";
  let path = input_path(relative_path);

  let contents = std::fs::read_to_string(path).unwrap();
  let lines = contents.split("\n").collect::<Vec<&str>>();
  let report = lines
      .iter()
      .map(|&x| x.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
      .collect::<Vec<Vec<u32>>>();

  let res1 = calc_consumption(&report);
  println!("the power consumption of the submarine is {}", res1);

  let res2 = life_support_rating(&report);
  println!("the life support rating of the submarine is {}", res2);
}

#[cfg(test)]
mod tests {
  #[test]
  fn aoc2021_day3() {
    super::binary_diagnostic();
  }
}