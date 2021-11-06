use unit;
mod helper;
use helper::compare_files;
use std::path::Path;

#[test]
fn it_adds_test_for_elixir_files() {
  let examples_path = helper::get_examples_path().join("elixir/std");
  let sample_path = Path::new("lib/app/user.ex");

  unit::run(&examples_path, &sample_path, "std").unwrap();

  let generated_path = examples_path.join("test/app/user_test.exs");
  let expected_path = examples_path.join("test.expected/app/user_test.exs");

  compare_files(&generated_path, &expected_path);
}
