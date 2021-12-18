type Map = Vec<Vec<i32>>;

fn read_map() -> Map {
  let relative_path = "/src/aoc2021/day9/input";
  let path = crate::utils::input_path(relative_path);

  let contents = std::fs::read_to_string(path).unwrap();

  contents
    .split("\n")
    .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
    .collect()
}

fn is_lowest(mp: &Map, i: usize, j: usize) -> bool {
  let mut a = 10;
  let mut b = 10;
  let mut c = 10;
  let mut d = 10;
  let cur = mp[i][j];

  if i > 0 {
    a = mp[i - 1][j];
  }
  if j > 0 {
    b = mp[i][j - 1];
  }
  if j + 1 < mp[i].len() {
    c = mp[i][j + 1];
  }
  if i + 1 < mp.len() {
    d = mp[i + 1][j];
  }

  cur < a && cur < b && cur < c && cur < d
}

fn calc_lowest(mp: &Map) -> i32 {
  let mut ret =  0;

  for i in 0..mp.len() {
    for j in 0..mp[i].len() {
      if is_lowest(mp, i, j) {
        ret += mp[i][j] + 1;
      }
    }
  }

  ret
}

fn dfs(mp: &Map, vis: &mut Map, x: i32, y: i32) -> i32 {
  let mut ret = 0;

  let dir = vec![vec![0, -1], vec![-1, 0], vec![0, 1], vec![1, 0]];
  for d in &dir {
    let xx = x + d[0];
    let yy = y + d[1];
    if xx >= 0 && xx < (mp.len() as i32) && yy >= 0 && yy < mp[xx as usize].len() as i32 && vis[xx as usize][yy as usize] != 1 {
      let t = mp[xx as usize][yy as usize];
      if t != 9 && t > mp[x as usize][y as usize] {
        vis[xx as usize][yy as usize] = 1;
        ret += dfs(mp, vis, xx, yy) + 1;
      }
    }
  }

  ret
}

fn calc_basin(mp: &Map) -> i32 {
  let mut vis = Vec::new();
  vis.resize(mp.len(), Vec::new());
  for i in 0..mp.len() {
    vis[i].resize(mp[i].len(), 0);
  }

  let mut vec = Vec::new();

  for i in 0..mp.len() {
    for j in 0..mp[i].len() {
      if is_lowest(mp, i, j) && vis[i][j] == 0 {
        vis[i][j] = 1;
        let res = dfs(mp, &mut vis, i as i32, j as i32);
        if res != 0 {
          vec.push(res + 1);
        }
      }
    }
  }

  vec.sort_by(|a, b| b.cmp(a));
  vec[0] * vec[1] * vec[2]
}

pub fn smoke_basin() {
  let mp = read_map();

  let res1 = calc_lowest(&mp);
  let res2 = calc_basin(&mp);
  println!("the sum of the risk levels of all low points is {}", res1);
  println!("the multiply of three largest basins is {}", res2);
}

#[cfg(test)]
mod tests {
  #[test]
  fn aoc2021_day10() {
    super::smoke_basin();
  }
}