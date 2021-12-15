pub fn input_path(relative_path: &str) -> String {
  let current_dir = std::env::current_dir().expect("Unable to get current dir");
  let path = String::from(current_dir.to_str().unwrap()) + relative_path;
  return path;
}

#[macro_export]
macro_rules! method_map {
  ($ ($year: expr => $day: expr => $func: expr), *) => {
    {
    let mut methods = std::collections::HashMap::new();
    $(
      let mut func_name = String::from("aoc");
      func_name.push_str(&$year.to_string());
      func_name.push_str("_day");
      func_name.push_str(&$day.to_string());
      methods.insert(func_name, $func as fn());
    )*
    methods
    }
  };
}

// TODO: method_map_2021
