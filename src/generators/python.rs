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
    format!("python -m unittest {}", &test_path.display())
  }

  fn create_test(&self, root: &Path, path: &Path) -> Result<(), Box<dyn Error>> {
    let (child_path, file_stem, path_without_extension, _) = self.path_destructing(&path);

    let test_folder = root.join("tests").join(child_path);
    let test_file_name = format!("test_{}.py", file_stem);
    let test_path = test_folder.join(test_file_name);
    self.bail_if_existing(&test_path)?;

    fs::create_dir_all(test_folder).unwrap_or_default();
    fs::write(
      &test_path,
      format!(
        r#"import unittest

import {}

class {}TestCase(unittest.TestCase):
    def test_it_works(self):
        self.assertEqual(1 + 1, 2)
"#,
        path_without_extension.to_str().unwrap().replace("/", "."),
        file_stem.to_class_case()
      ),
    )?;

    self.success_message(&test_path);
    Ok(())
  }
}
