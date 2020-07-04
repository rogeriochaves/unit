use unit;
mod helper;
use helper::compare_files;
use std::path::Path;

#[test]
fn it_adds_test_for_ruby_files() {
  let examples_path = helper::get_examples_path().join("ruby/std");
  let sample_path = Path::new("app/user.rb");

  unit::run(&examples_path, &sample_path).unwrap();

  let generated_path = examples_path.join("test/test_user.rb");
  let expected_path = examples_path.join("test.expected/test_user.rb");

  compare_files(&generated_path, &expected_path);
}

// #[test]
// fn it_does_not_add_test_for_ruby_files_twice() {
//   let examples_path = helper::get_examples_path();
//   let sample_path = Path::new("ruby/std/lib.rs");
//   let expected_path = examples_path.join("ruby/std/lib.expected.rs");

//   unit::run(&sample_path).unwrap();
//   unit::run(&sample_path).unwrap();

//   compare_files(&sample_path, &expected_path);
// }

#[test]
fn it_adds_test_for_nested_ruby_files() {
  let examples_path = helper::get_examples_path().join("ruby/std");
  let sample_path = Path::new("app/controllers/user_controller.rb");

  unit::run(&examples_path, &sample_path).unwrap();

  let generated_path = examples_path.join("test/controllers/test_user_controller.rb");
  let expected_path = examples_path.join("test.expected/controllers/test_user_controller.rb");

  compare_files(&generated_path, &expected_path);
}
