use unit;
mod helper;
use helper::compare_files;
use std::path::Path;

#[test]
fn it_adds_junit_test_for_java_files() {
  let examples_path = helper::get_examples_path().join("java/junit");
  let sample_path = Path::new("src/main/java/com/example/FooBar.java");

  unit::run(&examples_path, &sample_path, "junit").unwrap();

  let generated_path = examples_path.join("src/test/java/com/example/FooBarTest.java");
  let expected_path = examples_path.join("src/test.expected/java/com/example/FooBarTest.java");

  compare_files(&generated_path, &expected_path);

  let generated_pom_xml = examples_path.join("pom.xml");
  let expected_pom_xml = examples_path.join("pom.expected.xml");

  compare_files(&generated_pom_xml, &expected_pom_xml);
}
