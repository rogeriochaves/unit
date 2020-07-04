use unit;
mod helper;
use helper::compare_files;

#[test]
fn it_adds_test_for_rust_files() {
  let examples_path = helper::get_examples_path();
  let sample_path = examples_path.join("rust/lib.rs");
  let expected_path = examples_path.join("rust/lib.expected.rs");

  unit::run(&sample_path).unwrap();

  compare_files(&sample_path, &expected_path);
}

#[test]
fn it_does_not_add_test_for_rust_files_twice() {
  let examples_path = helper::get_examples_path();
  let sample_path = examples_path.join("rust/lib.rs");
  let expected_path = examples_path.join("rust/lib.expected.rs");

  unit::run(&sample_path).unwrap();
  unit::run(&sample_path).unwrap();

  compare_files(&sample_path, &expected_path);
}
