use std::collections::{HashMap, HashSet};

type Graph = HashMap<String, Vec<String>>;

fn build_graph() -> Graph {
  let relative_path = "/src/aoc2021/day12/input";
  let path = crate::utils::input_path(relative_path);

  let content = std::fs::read_to_string(path).unwrap();

  let mut graph = Graph::new();

  content
    .split("\n")
    .for_each(|line| {
      let v = line.split("-").collect::<Vec<_>>();
      let start = String::from(v[0]);
      let end = String::from(v[1]);
      graph.entry(start.clone()).or_default().push(end.clone());
      graph.entry(end).or_default().push(start);
    });

  graph
}

fn dfs(graph: &Graph, cur: &String, visited: &mut HashSet<String>, times: &mut i32) {
  if cur == "end" {
    *times += 1;
    return;
  }

  let candidates = graph.get(cur).unwrap();
  for c in candidates {
    if c == "start" {
      continue;
    }
    let is_lower = c.chars().next().unwrap().is_lowercase();
    if !visited.contains(c)  {
      if is_lower {
        visited.insert(c.clone());
      }
      dfs(graph, c, visited, times);
      visited.remove(c);
    }
  }
}

fn dfs2(graph: &Graph, cur: &String, visited: &mut HashMap<String, i32>, times: &mut i32, max_n: i32) {
  if cur == "end" {
    *times += 1;
    return;
  }

  let candidates = graph.get(cur).unwrap();
  for c in candidates {
    if c == "start" {
      continue;
    }
    let other_count = visited
      .iter()
      .filter(|(a, _)| *a != c)
      .filter(|(_, &x)| x >= max_n)
      .count();
    let cur_count = *visited.get(c).unwrap_or(&0);
    let is_lower = c.chars().next().unwrap().is_lowercase();
    if !is_lower || (cur_count < max_n - 1 && other_count <= 1) || (cur_count < max_n && other_count == 0) {
      if is_lower {
        *visited.entry(c.clone()).or_insert(0) += 1;
      }
      dfs2(graph, c, visited, times, max_n);
      if is_lower {
        *visited.entry(c.clone()).or_insert(0) -= 1;
      }
    }
  }
}

fn find_path(graph: &Graph, n: i32) -> i32 {
  let mut times = 0;
  if n == 1 {
    let mut visited = HashSet::new();
    visited.insert(String::from("start"));
    dfs(graph, &String::from("start"), &mut visited, &mut times);
  } else {
    let mut visited = HashMap::new();
    visited.insert(String::from("start"), 1);
    dfs2(graph, &String::from("start"), &mut visited, &mut times, 2);
  }
  times
}

pub fn passage_pathing() {
  let graph = build_graph();

  let res1 = find_path(&graph, 1);
  let res2 = find_path(&graph, 2);
  println!("{} paths", res1);
  println!("{} paths for new rule", res2);
}

#[cfg(test)]
mod tests {
  #[test]
  fn aoc2021_day12() {
    super::passage_pathing();
  }
}