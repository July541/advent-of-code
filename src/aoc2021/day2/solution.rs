use crate::utils::input_path;
use num_bigint::BigUint;

type OutType = BigUint;

enum Part {
  One, Two
}

fn render_route_part1(cmd: &(&str, OutType), pos: ((OutType, OutType), OutType)) -> ((OutType, OutType), OutType) {
  match cmd {
    ("forward", n) => { return ((pos.0.0, pos.0.1 + n), pos.1); }
    ("up", n) => { return ((pos.0.0 - n, pos.0.1), pos.1); }
    ("down", n) => { return ((pos.0.0 + n, pos.0.1), pos.1); }
    _ => panic!("Unexpected cmd")
  }
}

fn render_route_part2(cmd: &(&str, OutType), pos: ((OutType, OutType), OutType)) -> ((OutType, OutType), OutType) {
  match cmd {
    ("forward", n) => {
      let aim = pos.1.clone();
      return ((pos.0.0 + pos.1 * n, pos.0.1 + n), aim);
    }
    ("up", n) => { return (pos.0, pos.1 - n); }
    ("down", n) => { return (pos.0, pos.1 + n); }
    _ => panic!("Unexpected cmd")
  }
}

fn final_depth(routes: &Vec<(&str, OutType)>, part: Part) -> (OutType, OutType) {
  // ((depth, horizontal), aim)
  let mut pos = ((BigUint::default(), BigUint::default()), BigUint::default());

  for cmd in routes {
    match part {
      Part::One => { pos = render_route_part1(&cmd, pos); }
      Part::Two => { pos = render_route_part2(&cmd, pos); }
    }
  }

  return pos.0;
}

pub fn dive() {
  let relative_path = "/src/aoc2021/day2/input";
  let path = input_path(relative_path);

  let contents = std::fs::read_to_string(path).unwrap();
  let lines = contents.split_terminator("\n").collect::<Vec<&str>>();
  let routes = lines.iter()
      .map(|&x| x.split(" ")
      .collect::<Vec<&str>>())
      .map(|x: Vec<&str>| (x[0], x[1].parse().unwrap())) //get command and value
      .collect::<Vec<(&str, OutType)>>();

  // Part 1
  let res1 = final_depth(&routes, Part::One);
  println!("multiply final horizontal position by final depth is: {}", res1.0 * res1.1);

  // Part2
  let res2 = final_depth(&routes, Part::Two);
  println!("multiply final horizontal position by final depth for new manual: {}", res2.0 * res2.1);
}

#[cfg(test)]
mod tests {
  use super::dive;
  #[test]
  fn aoc2021_day2() {
    dive();
  }
}