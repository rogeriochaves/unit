use unit;
mod helper;
use helper::compare_files;
use std::path::Path;
use std::fs;

#[test]
fn it_adds_elm_test_for_elm_files() {
  let examples_path = helper::get_examples_path().join("elm/elm-test");
  let sample_path = Path::new("src/Cats.elm");

  unit::run(&examples_path, &sample_path, "elm-test").unwrap();

  let generated_path = examples_path.join("tests/CatsTest.elm");
  let expected_path = examples_path.join("tests.expected/CatsTest.elm");

  compare_files(&generated_path, &expected_path);

  let generated_elm_json = examples_path.join("elm.json");
  let elm_json = fs::read_to_string(&generated_elm_json).unwrap();
  assert!(elm_json.contains("elm-explorations/test"));
}
