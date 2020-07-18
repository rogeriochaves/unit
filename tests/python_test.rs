use unit;
mod helper;
use helper::compare_files;
use std::path::Path;

#[test]
fn it_adds_test_for_python_files() {
  let examples_path = helper::get_examples_path().join("python/std");
  let sample_path = Path::new("app/user.py");

  unit::run(&examples_path, &sample_path, "std").unwrap();

  let generated_path = examples_path.join("tests/test_user.py");
  let expected_path = examples_path.join("tests.expected/test_user.py");

  compare_files(&generated_path, &expected_path);
}
