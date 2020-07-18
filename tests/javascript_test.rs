use unit;
mod helper;
use helper::compare_files;
use std::path::Path;
extern crate regex;
use regex::Regex;
use std::fs;

#[test]
fn it_adds_jest_test_for_javascript_files() {
  let examples_path = helper::get_examples_path().join("javascript/jest");
  let sample_path = Path::new("src/sum.js");

  unit::run(&examples_path, &sample_path, "jest").unwrap();

  let generated_path = examples_path.join("src/sum.test.js");
  let expected_path = examples_path.join("src/sum.test.expected.js");

  compare_files(&generated_path, &expected_path);

  let generated_package_json = examples_path.join("package.json");
  let expected_package_json = examples_path.join("package.expected.json");

  let mut package_json = fs::read_to_string(&generated_package_json).unwrap();
  let re = Regex::new(r#""jest": ".*""#).unwrap();
  package_json = re
    .replace_all(&package_json, r#""jest": "VERSION""#)
    .to_string();
  fs::write(&generated_package_json, package_json).unwrap();

  compare_files(&generated_package_json, &expected_package_json);
}
