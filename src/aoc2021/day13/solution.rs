use itertools::Itertools;

type Origami = Vec<Vec<bool>>;

#[derive(Clone)]
enum Dir {
  X, Y
}

type FoldRule = Vec<(Dir, usize)>;

fn read_origami() -> Origami {
  let relative_path = "/src/aoc2021/day13/input1";
  let path = crate::utils::input_path(relative_path);
  
  let contents = std::fs::read_to_string(path).unwrap();

  let pos = contents
    .split("\n")
    .map(|line| {
      let row = line.split(",").collect::<Vec<_>>();
      let y = row[0].parse::<usize>().unwrap();
      let x = row[1].parse::<usize>().unwrap();
      (y, x)
    })
    .collect::<Vec<_>>();

  let max_y = 1 + pos
    .iter()
    .map(|x| x.0)
    .max()
    .unwrap();

  let max_x = 1 + pos
    .iter()
    .map(|x| x.1)
    .max()
    .unwrap();

  let mut paper = Vec::new();
  paper.resize(max_x, Vec::new());
  paper.iter_mut()
    .for_each(|row| row.resize(max_y, false));

  for (y, x) in pos {
    paper[x][y] = true;
  }

  paper
}

fn read_fold() -> FoldRule {
  let relative_path = "/src/aoc2021/day13/input2";
  let path = crate::utils::input_path(relative_path);

  let contents = std::fs::read_to_string(path).unwrap();

  let mut rules = FoldRule::new();

  for line in contents.split("\n") {
    if line.contains("x") {
      let x = String::from(line.chars().dropping(13).as_str()).parse::<usize>().unwrap();
      rules.push((Dir::X, x));
    } else {
      let y = String::from(line.chars().dropping(13).as_str()).parse::<usize>().unwrap();
      rules.push((Dir::Y, y));
    }
  }

  rules
}

fn fold_y(paper: &Origami, y: usize) -> Origami {
  let mut new = paper.clone();

  for i in 0..y {
    for j in 0..new[i].len() {
      new[i][j] = new[i][j] || new[new.len() - i - 1][j];
    }
  }

  new = new
    .into_iter()
    .take(y)
    .collect::<Vec<_>>();

  new
}

fn fold_x(paper: &Origami, x: usize) -> Origami {
  let mut new = paper.clone();

  for i in 0..new.len() {
    for j in 0..x {
      new[i][j] = new[i][j] || new[i][new[i].len() - j - 1];
    }
  }

  new = new
    .into_iter()
    .map(|row| row.into_iter().take(x).collect::<Vec<_>>())
    .collect::<Vec<_>>();

  new
}

fn count_dots(paper: &Origami) -> i32 {
  paper
    .iter()
    .map(|row| row.iter().map(|&x| if x {1} else {0}).sum::<i32>())
    .sum()
}

fn dbg(paper: &Origami) {
  paper
    .iter()
    .for_each(|row| {
      row.iter().for_each(|&b| if b {print!("#");} else {print!(".")});
      println!();
    });
  println!();
}

fn fold(paper: &Origami, rules: &FoldRule) -> Origami {
  let mut cur = paper.clone();
  rules
    .iter()
    .for_each(|(dir, size)| {
      match dir {
        Dir::X => cur = fold_x(&cur, *size),
        Dir::Y => cur = fold_y(&cur, *size)
      }
    });
  
  cur
}

pub fn transparent_origami() {
  let paper = read_origami();
  let rules = read_fold();

  let res1 = count_dots(&fold(&paper, &rules.clone().into_iter().take(1).collect::<Vec<_>>()));
  let res2 = fold(&paper, &rules);

  println!("there are {} dots", res1);
  dbg(&res2);
}

#[cfg(test)]
mod tests {
  #[test]
  fn aoc2021_day13() {
    super::transparent_origami();
  }
}