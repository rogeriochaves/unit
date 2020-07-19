use super::super::Generator;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
extern crate cmd_lib;
use cmd_lib::run_cmd;
extern crate regex;
use regex::Regex;

pub struct Jest();

impl Generator for Jest {
  fn option_name(&self) -> &'static str {
    "jest"
  }

  fn run_command(&self, _test_path: &Path) -> String {
    String::from("npm test")
  }

  fn setup(&self, root: &Path) -> Result<(), Box<dyn Error>> {
    let package_json_path = root.join("package.json");
    let mut package_json = fs::read_to_string(&package_json_path)?;

    if !package_json.contains("jest") {
      let current_dir = env::current_dir()?;
      if root.to_str().unwrap() != "" {
        env::set_current_dir(root.to_str().unwrap())?;
      }
      if root.join("yarn.lock").exists() {
        run_cmd!("yarn add jest --dev").unwrap();
      } else {
        run_cmd!("npm install --save-dev jest").unwrap();
      }
      env::set_current_dir(current_dir)?;

      package_json = fs::read_to_string(&package_json_path)?;
      if !package_json.contains(r#""test":"#) {
        package_json =
          package_json.replace(r#""scripts": {"#, "\"scripts\": {\n    \"test\": \"jest\"");
      } else if !package_json.contains(r#""test": "jest"#) {
        let re = Regex::new(r#""test": ".*""#).unwrap();
        package_json = re
          .replace_all(&package_json, r#""test": "jest""#)
          .to_string();
      }
      fs::write(&package_json_path, package_json)?;
    }

    Ok(())
  }

  fn create_test(&self, root: &Path, path: &Path) -> Result<(), Box<dyn Error>> {
    self.setup(root)?;

    let (_, file_stem, _, _) = self.path_destructing(&path);

    let full_path = root.join(path);
    let file_folder = full_path.parent().unwrap();
    let test_file_name = format!("{}.test.js", file_stem);
    let test_path = file_folder.join(test_file_name);
    self.bail_if_existing(&test_path)?;

    fs::write(
      &test_path,
      format!(
        r#"const {} = require("./{}");

test("it works", () => {{
  expect(1 + 1).toBe(2);
}});
"#,
        file_stem, file_stem
      ),
    )?;

    self.success_message(&test_path);
    Ok(())
  }
}
