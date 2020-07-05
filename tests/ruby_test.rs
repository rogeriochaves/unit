use unit;
mod helper;
use helper::compare_files;
use std::fs;
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

#[test]
fn it_does_not_overwrite_existing_test_for_ruby_files() {
  let examples_path = helper::get_examples_path().join("ruby/std");
  let sample_path = Path::new("app/user.rb");

  unit::run(&examples_path, &sample_path).unwrap();

  let generated_path = examples_path.join("test/test_user.rb");
  let generated_content = fs::read_to_string(&generated_path).unwrap();
  fs::write(&generated_path, generated_content + "updated").unwrap();

  assert!(unit::run(&examples_path, &sample_path).is_err());

  let generated_content = fs::read_to_string(&generated_path).unwrap();
  assert!(generated_content.contains("updated"));
}

#[test]
fn it_adds_test_for_nested_ruby_files() {
  let examples_path = helper::get_examples_path().join("ruby/std");
  let sample_path = Path::new("app/controllers/user_controller.rb");

  unit::run(&examples_path, &sample_path).unwrap();

  let generated_path = examples_path.join("test/controllers/test_user_controller.rb");
  let expected_path = examples_path.join("test.expected/controllers/test_user_controller.rb");

  compare_files(&generated_path, &expected_path);
}
