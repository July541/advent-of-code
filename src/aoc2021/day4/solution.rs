#[derive(Clone)]
struct Board {
  board: [[i32; 5]; 5],
  vis: [[bool; 5]; 5],
}

impl Board {
  pub fn from(vec: &Vec<Vec<i32>>) -> Board {
    let mut b = [[0; 5]; 5];
    for i in 0..5 {
      for j in 0..5 {
        b[i][j] = vec[i][j];
      }
    }
    Board {
      board: b,
      vis: [[false; 5]; 5],
    }
  }

  pub fn visit(&mut self, val: i32) {
    for i in 0..5 {
      for j in 0..5 {
        if self.board[i][j] == val {
          self.vis[i][j] = true;
          return;
        }
      }
    }
  }

  pub fn is_win(&self) -> bool {
    let row_win = self.vis
      .iter()
      .any(|row| row
        .iter()
        .all(|x| *x)
      );
    if row_win {
      return true;
    }

    for j in 0..5 {
      let mut col_win = true;
      for i in 0..5 {
        col_win &= self.vis[i][j];
      }
      if col_win {
        return true;
      }
    }

    false
  }

  pub fn score(&self, val: i32) -> i32 {
    let mut score = 0;

    for i in 0..5 {
      for j in 0..5 {
        if self.vis[i][j] == false {
          score += self.board[i][j];
        }
      }
    }

    score * val
  }
}

fn read_input() -> Vec<i32> {
  let relative_path = "/src/aoc2021/day4/input1";
  let path = crate::utils::input_path(relative_path);

  let contents = std::fs::read_to_string(path).unwrap();
  let input = contents
    .split(",")
    .collect::<Vec<&str>>()
    .iter()
    .map(|x| x.parse().unwrap())
    .collect::<Vec<i32>>();

  input
}

fn read_board() -> Vec<Vec<Vec<i32>>> {
  let relative_path = "/src/aoc2021/day4/input2";
  let path = crate::utils::input_path(relative_path);

  let contents = std::fs::read_to_string(path).unwrap();

  let mut ret = Vec::new();
  let board = &mut Vec::new();

  for line in contents.lines() {
    if line == "" {
      continue;
    }

    let row = line
      .split(" ")
      .filter(|x| *x != "")
      .map(|x| x.parse().unwrap())
      .collect::<Vec<i32>>();
    board.push(row);

    if board.len() == 5 {
      ret.push(board.clone());
      board.clear();
    }
  }

  ret
}

fn play_bingo_first_win(mut boards: Vec<Board>, input: &Vec<i32>) -> Option<i32> {
  let mut score = None;

  for &i in input {
    boards
      .iter_mut()
      .for_each(|board| {
        board.visit(i);
        if board.is_win() {
          score = Some(board.score(i));
        }
      });
    match score {
      Some(_) => return score,
      None => continue
    }
  }
  score
}

fn play_bingo_last_win(mut boards: Vec<Board>, input: &Vec<i32>) -> Option<i32> {
  let mut score = None;

  for &i in input {
    boards
      .iter_mut()
      .for_each(|board| {
        if !board.is_win() {
          board.visit(i);
          if board.is_win() {
            score = Some(board.score(i));
          }
        }
    });
  }

  score
}

pub fn giant_squid() {
  let input = read_input();
  let boards = read_board()
    .iter()
    .map(|board| Board::from(board))
    .collect::<Vec<Board>>();

  let boards2 = boards.clone();

  let score1 = play_bingo_first_win(boards, &input).unwrap();
  let score2 = play_bingo_last_win(boards2, &input).unwrap();
  println!("final score of choose current board: {}", score1);
  println!("final score of last board: {}", score2);
}

#[cfg(test)]
mod tests {
  #[test]
  fn aoc2021_day4() {
    super::giant_squid()
  }
}
