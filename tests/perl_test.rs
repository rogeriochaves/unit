use unit;
mod helper;
use helper::compare_files;
use std::path::Path;

#[test]
fn it_adds_test_for_perl_files() {
  let examples_path = helper::get_examples_path().join("perl/std");
  let sample_path = Path::new("lib/Model/User.pm");

  unit::run(&examples_path, &sample_path, "std").unwrap();

  let generated_path = examples_path.join("t/Model/user_somecase.t");
  let expected_path = examples_path.join("t.expected/Model/user_somecase.t");

  compare_files(&generated_path, &expected_path);
}

#[test]
fn it_fails_it_perl_module_is_not_in_lib() {
  let examples_path = helper::get_examples_path().join("perl/std");
  let sample_path = Path::new("src/Model/User.pm");

  let result = unit::run(&examples_path, &sample_path, "std");

  assert!(result.is_err());
}
