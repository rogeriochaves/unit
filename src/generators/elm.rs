use super::super::Generator;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
extern crate cmd_lib;
use cmd_lib::{run_cmd, run_fun};
extern crate regex;

pub struct ElmTest();

impl Generator for ElmTest {
  fn option_name(&self) -> &'static str {
    "elm-test"
  }

  fn run_command(&self, _test_path: &Path) -> String {
    String::from("elm-test")
  }

  fn setup(&self, root: &Path) -> Result<(), Box<dyn Error>> {
    let elm_json_path = root.join("elm.json");
    let elm_json = fs::read_to_string(&elm_json_path)?;

    if !elm_json.contains("elm-explorations/test") {
      run_cmd!("npm -g install elm-test").unwrap();
      let current_dir = env::current_dir()?;
      if root.to_str().unwrap() != "" {
        env::set_current_dir(root.to_str().unwrap())?;
      }
      let prefix = run_fun!(npm config get prefix)?;
      run_cmd!("yes | {}/bin/elm-test init", prefix).unwrap();
      run_cmd!("rm tests/Example.elm").unwrap();
      env::set_current_dir(current_dir)?;
    }

    Ok(())
  }

  fn create_test(&self, root: &Path, path: &Path) -> Result<(), Box<dyn Error>> {
    self.setup(root)?;

    let (child_path, file_stem, _, _) = self.path_destructing(&path);

    let test_folder = root.join("tests").join(child_path);
    let test_file_name = format!("{}Test.elm", file_stem);
    let test_path = test_folder.join(test_file_name);
    self.bail_if_existing(&test_path)?;

    fs::write(
      &test_path,
      format!(
        r#"module {}Test exposing (suite)

import {}
import Expect
import Test exposing (..)


suite : Test
suite =
    test "it works" <|
        \_ ->
            (1 + 1)
                |> Expect.equal 2
"#,
        file_stem, file_stem
      ),
    )?;

    self.success_message(&test_path);
    Ok(())
  }
}
