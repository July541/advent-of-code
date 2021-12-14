pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

use std::collections::HashMap;

use self::day1::solution::sonar_sweep;
use self::day2::solution::dive;
use self::day3::solution::binary_diagnostic;
use self::day4::solution::giant_squid;

pub fn methods() -> HashMap<String, fn()> {
  let mut methods = HashMap::new();
  methods.insert(String::from("aoc2021_day1"), sonar_sweep as fn());
  methods.insert(String::from("aoc2021_day2"), dive as fn());
  methods.insert(String::from("aoc2021_day3"), binary_diagnostic as fn());
  methods.insert(String::from("aoc2021_day4"), giant_squid as fn());
  return methods;
}
