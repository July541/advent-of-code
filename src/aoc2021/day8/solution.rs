use itertools::Itertools;

fn read_data() -> Vec<(Vec<String>, Vec<String>)> {
  let relative_path = "/src/aoc2021/day8/input";
  let path = crate::utils::input_path(relative_path);

  let contents = std::fs::read_to_string(path).unwrap();

  contents
    .split("\n")
    .map(|line| {
      let v = line.split(" | ").collect::<Vec<_>>();
      (
        v[0].split(" ").map(|s| String::from(s)).collect()
      , v[1].split(" ").map(|s| String::from(s)).collect()
      )
    })
    .collect()
}

fn calc_digit(v: &Vec<(Vec<String>, Vec<String>)>) -> i32 {
  let mut cnt = 0;

  v.iter()
    .for_each(|(_, b)| b.iter().for_each(|s| {
      let len = s.len();
      if len == 2 || len == 3 || len == 4 || len == 7 {
        cnt += 1;
      }
    }));

  cnt
}

fn try_restore(v: &Vec<(Vec<String>, Vec<String>)>) -> i32 {

  let base = ('a'..='g').collect_vec();
  let pattern = vec![
    "012456"
  , "25"
  , "02346"
  , "02356"
  , "1235"
  , "01356"
  , "013456"
  , "025"
  , "0123456"
  , "012356"
  ];

  let mut cnt = 0;

  v.iter()
    .for_each(|(a, b)| {
      base.iter().permutations(7).for_each(|s| {
        let r = a.iter().map(|t| {
          let r = t.chars()
            .map(|c| ((s.iter().find_position(|&&&c1| c == c1).unwrap().0 as u8) + 48) as char)
            .sorted()
            .collect::<String>();
          pattern.contains(&r.as_str())
        }).all(|x| x);
        if r {
          let res: i32 = b.iter().map(|t| {
            let r = t.chars()
              .map(|c| ((s.iter().find_position(|&&&c1| c == c1).unwrap().0 as u8) + 48) as char)
              .sorted()
              .collect::<String>();
            pattern.iter().find_position(|x| **x == r.as_str()).unwrap().0 as i32
          }).fold1(|x, y| x * 10 + y).unwrap();
          cnt += res;
        }
      })
    });

  cnt
}

pub fn seven_segment_search() {
  let data = read_data();

  let res1 = calc_digit(&data);
  let res2 = try_restore(&data);
  println!("appear {} times.", res1);
  println!("the sum is {}", res2);
}

#[cfg(test)]
mod tests {
  #[test]
  fn aoc2021_day8() {
    super::seven_segment_search();
  }
}
