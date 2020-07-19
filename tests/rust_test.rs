use unit;
mod helper;
use helper::compare_files;
use std::path::Path;

#[test]
fn it_adds_test_for_rust_files() {
  let examples_path = helper::get_examples_path();
  let sample_path = examples_path.join("rust/std/lib.rs");
  let expected_path = examples_path.join("rust/std/lib.expected.rs");

  unit::run(Path::new(""), &sample_path, "std").unwrap();

  compare_files(&sample_path, &expected_path);
}

#[test]
fn it_does_not_add_test_for_rust_files_twice() {
  let examples_path = helper::get_examples_path();
  let sample_path = examples_path.join("rust/std/lib.rs");
  let expected_path = examples_path.join("rust/std/lib.expected.rs");

  unit::run(Path::new(""), &sample_path, "std").unwrap();
  assert!(unit::run(Path::new(""), &sample_path, "std").is_err());

  compare_files(&sample_path, &expected_path);
}

#[test]
fn it_adds_integration_test_for_rust_files() {
  let examples_path = helper::get_examples_path().join("rust/integration");
  let sample_path = Path::new("lib/module/user.rs");

  unit::run(&examples_path, &sample_path, "integration").unwrap();

  let generated_path = examples_path.join("tests/module/user_test.rs");
  let expected_path = examples_path.join("tests.expected/module/user_test.rs");

  compare_files(&generated_path, &expected_path);
}
