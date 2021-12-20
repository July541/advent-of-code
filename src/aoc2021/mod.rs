pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;

use std::collections::HashMap;
use crate::method_map;

pub fn methods() -> HashMap<String, fn()> {
  method_map![
    2021 => 1 => day1::sonar_sweep
  , 2021 => 2 => day2::dive
  , 2021 => 3 => day3::binary_diagnostic
  , 2021 => 4 => day4::giant_squid
  , 2021 => 5 => day5::hydrothermal_venture
  , 2021 => 6 => day6::lantern_fish
  , 2021 => 7 => day7::the_treachery_of_whales
  , 2021 => 8 => day8::seven_segment_search
  , 2021 => 9 => day9::smoke_basin
  , 2021 => 10 => day10::syntax_scoring
  , 2021 => 11 => day11::dumbo_octopus
  , 2021 => 12 => day12::passage_pathing
  , 2021 => 13 => day13::transparent_origami
  ]
}
