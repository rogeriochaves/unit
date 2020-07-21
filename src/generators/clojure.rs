use super::super::Generator;
use std::error::Error;
use std::fs;
use std::path::Path;
extern crate colored;
use colored::*;

extern crate cmd_lib;

pub struct Std();

impl Generator for Std {
  fn option_name(&self) -> &'static str {
    "std"
  }

  fn run_command(&self, test_path: &Path) -> String {
    if Path::new("project.clj").exists() {
      return String::from("lein test");
    }

    println!("{}", "Heads up! Your project does not seem to use leiningen, so you may have to add the test folder to your classpath in deps.edn like `{:paths [\"src\" \"test\"]}`\n".yellow());

    let test_module = test_path
      .to_str()
      .unwrap()
      .replace("test/", "")
      .replace("/", ".")
      .replace("_", "-")
      .replace(".clj", "");

    return format!(
      r#"=> (use 'clojure.test)
=> (use '{})
=> (run-all-tests)

in the clojure repl"#,
      test_module
    );
  }

  fn create_test(&self, root: &Path, path: &Path) -> Result<(), Box<dyn Error>> {
    let (child_path, file_stem, _, _) = self.path_destructing(&path);

    let test_folder = root.join("test").join(child_path);
    let test_file_name = format!("{}_test.clj", file_stem);
    let test_path = test_folder.join(test_file_name);
    self.bail_if_existing(&test_path)?;

    let clojure_module = child_path
      .join(&file_stem)
      .to_str()
      .unwrap()
      .replace("/", ".")
      .replace("_", "-");
    fs::create_dir_all(test_folder).unwrap_or_default();
    fs::write(
      &test_path,
      format!(
        r#"(ns {}-test
  (:require [clojure.test :refer :all]
            [{} :refer :all]))

(deftest example-test
  (testing "It works"
    (is (= 2 (+ 1 1)))))
"#,
        clojure_module, clojure_module,
      ),
    )?;

    self.success_message(&test_path);
    Ok(())
  }
}
