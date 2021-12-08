pub fn input_path(relative_path: &str) -> String {
  let current_dir = std::env::current_dir().expect("Unable to get current dir");
  let path = String::from(current_dir.to_str().unwrap()) + relative_path;
  return path;
}