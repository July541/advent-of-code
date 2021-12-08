use std::collections::HashMap;

pub mod day1;
pub mod day2;
// use day1::solution::sonar_sweep;

pub fn f<F>() -> HashMap<String, F> where F: Fn() {
  let methods = HashMap::new();
  // methods.insert(String::from("aoc2021_day1"), || {sonar_sweep();});

  return wrapper(methods);
}

pub fn wrapper<F: Fn()>(f: HashMap<String, F>) -> HashMap<String, F> {
  return f
}
