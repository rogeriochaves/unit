use super::super::Generator;
use std::error::Error;
use std::fs;
use std::path::Path;
extern crate inflector;
use inflector::Inflector;

extern crate cmd_lib;

pub struct Std();

impl Generator for Std {
  fn option_name(&self) -> &'static str {
    "std"
  }

  fn run_command(&self, test_path: &Path) -> String {
    format!("mix test {}", &test_path.display())
  }

  fn create_test(&self, root: &Path, path: &Path) -> Result<(), Box<dyn Error>> {
    let (child_path, file_stem, _, _) = self.path_destructing(&path);

    let test_folder = root.join("test").join(child_path);
    let test_file_name = format!("{}_test.exs", file_stem);
    let test_path = test_folder.join(test_file_name);
    self.bail_if_existing(&test_path)?;

    let module = child_path
      .join(&file_stem)
      .to_str()
      .unwrap()
      .split("/")
      .map(|x| x.to_class_case())
      .collect::<Vec<String>>()
      .join(".");

    fs::create_dir_all(test_folder).unwrap_or_default();
    fs::write(
      &test_path,
      format!(
        r#"defmodule {}Test do
  use ExUnit.Case
  alias {}

  test "it works" do
    assert 1 + 1 == 2
  end
end
"#,
        module,
        module,
      ),
    )?;

    self.success_message(&test_path);
    Ok(())
  }
}
