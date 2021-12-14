use std::cmp::Ordering::{Equal, Greater, Less};

#[derive(Debug)]
struct Line {
  from: (i32, i32),
  to: (i32, i32)
}

impl Line {
  fn is_horizontal(&self) -> bool {
    self.from.0 == self.to.0
  }

  fn is_vertical(&self) -> bool {
    self.from.1 == self.to.1
  }

  pub fn is_diagonal(&self) -> bool {
    (self.from.0 - self.to.0).abs() == (self.from.1 - self.to.1).abs()
  }
}

struct Board {
  board: [[i32; 1000]; 1000]
}

impl Board {
  pub fn new() -> Board {
    Board { board: [[0; 1000]; 1000] }
  }

  pub fn produce(&mut self, line: &Line) {
    if line.is_horizontal() {
      let x = line.from.0 as usize;
      let l = line.from.1.min(line.to.1) as usize;
      let r = line.from.1.max(line.to.1) as usize;
      for y in l ..= r {
        self.board[y][x] += 1;
      }
    }
    if line.is_vertical() {
      let y = line.from.1 as usize;
      let l = line.from.0.min(line.to.0) as usize;
      let r = line.from.0.max(line.to.0) as usize;
      for x in l ..= r {
        self.board[y][x] += 1;
      }
    }
  }

  pub fn produce_diagonal(&mut self, line: &Line) {
    let (x1, y1) = line.from;
    let (x2, y2) = line.to;

    let mut target: Vec<(usize, usize)> = Vec::new();

    match (y1.cmp(&y2), x1.cmp(&x2)) {
      (Equal, Equal) => {
        target.push((y1 as usize, x1 as usize));
      },
      (Less, Less) => {
        let mut positions = (y1..=y2)
          .zip(x1..=x2)
          .map(|(a, b)| (a as usize, b as usize))
          .collect::<Vec<(usize, usize)>>();

        target.append(&mut positions);
      }
      (Less, Greater) => {
        let mut positions = (y1..=y2)
          .zip((x2..=x1).rev())
          .map(|(a, b)| (a as usize, b as usize))
          .collect::<Vec<_>>();

        target.append(&mut positions);
      }
      (Greater, Greater) => {
        let mut positions = (y2..=y1)
          .zip(x2..=x1)
          .map(|(a, b)| (a as usize, b as usize))
          .collect::<Vec<_>>();

        target.append(&mut positions);
      }
      (Greater, Less) => {
        let mut positions = ((y2..=y1).rev())
          .zip(x1..=x2)
          .map(|(a, b)| (a as usize, b as usize))
          .collect::<Vec<_>>();

        target.append(&mut positions);
      }
      _ => {}
    }

    for (y, x) in target {
      self.board[y][x] += 1;
    }
  }

  pub fn overlap_count(&self) -> i32 {
    let mut ret = 0;

    for i in 0 .. 1000 {
      for j in 0 .. 1000 {
        if self.board[j][i] > 1 {
          ret += 1;
        }
      }
    }

    ret
  }
}

fn read_lines() -> Vec<Line> {
  let relative_path = "/src/aoc2021/day5/input";
  let path = crate::utils::input_path(relative_path);

  let contents = std::fs::read_to_string(path).unwrap();
  let lines = contents
    .split("\n")
    .collect::<Vec<_>>()
    .iter()
    .map(
      |line| {
        let point = line.split(" -> ")
          .map(|pos| {
            let ps = pos.split(",").collect::<Vec<_>>();
            return (ps[0].parse::<i32>().unwrap(), ps[1].parse::<i32>().unwrap());
          })
          .collect::<Vec<_>>();
        return Line {from: point[0], to: point[1]}
      }
    )
    .collect::<Vec<_>>();

  lines
}

fn two_direction(lines: &Vec<Line>) -> i32 {
  let mut board = Board::new();

  for line in lines {
    board.produce(line);
  }

  board.overlap_count()
}

fn three_direction(lines: &Vec<Line>) -> i32 {
  let mut board = Board::new();

  for line in lines {
    board.produce(line);
    if line.is_diagonal() {
      board.produce_diagonal(line);
    }
  }

  board.overlap_count()
}

pub fn hydrothermal_venture() {
  let lines = read_lines();

  let res1 = two_direction(&lines);
  let res2 = three_direction(&lines);
  println!("{} points at least two lines overlap for two directions.", res1);
  println!("{} points at least two lines overlap for three directions.", res2);
}

#[cfg(test)]
mod tests {
  #[test]
  fn aoc2021_day5() {
    super::hydrothermal_venture()
  }
}