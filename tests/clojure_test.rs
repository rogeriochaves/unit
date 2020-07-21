use unit;
mod helper;
use helper::compare_files;
use std::path::Path;

#[test]
fn it_adds_test_for_clojure_files() {
  let examples_path = helper::get_examples_path().join("clojure/std");
  let sample_path = Path::new("src/myproject/welcome_helper.clj");

  unit::run(&examples_path, &sample_path, "std").unwrap();

  let generated_path = examples_path.join("test/myproject/welcome_helper_test.clj");
  let expected_path = examples_path.join("test.expected/myproject/welcome_helper_test.clj");

  compare_files(&generated_path, &expected_path);
}
