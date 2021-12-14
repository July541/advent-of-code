use std::process::exit;
use advent_of_code::aoc2021::methods as method_map;

enum ExitStatus {
  Success = 0
, MissingArgs = 2
, InvalidArgs = 127
}

fn safe_exit() {
  println!("Usage:\n\tcargo run <year> <day>\nExample:\n\tcargo run 2021 3");
  exit(ExitStatus::MissingArgs as i32);
}

pub fn main() {
  let args:Vec<_> = std::env::args().collect();
  if args.len() != 3 {
    safe_exit();
  }

  let year = &args[1];
  let parsed_day = &args[2].parse::<i32>(); // remove leading 0s

  match parsed_day {
    Ok(day) => {
      let day_str = day.to_string();
      let mut method_name = String::from("aoc");
      method_name.push_str(year);
      method_name.push_str("_day");
      method_name.push_str(&day_str);

      match method_map().get(&method_name) {
        Some(func) => {
          func();
          exit(ExitStatus::Success as i32);
        },
        None => {
          println!("Not found method: {}", method_name);
          exit(ExitStatus::InvalidArgs as i32)
        }
    }
    }
    Err(_) => safe_exit()
  }
}