use super::super::Generator;
use std::error::Error;
use std::fs;
use std::path::Path;
extern crate inflector;
use inflector::Inflector;
use simple_error::bail;

extern crate cmd_lib;

pub struct Std();

impl Generator for Std {
  fn option_name(&self) -> &'static str {
    "std"
  }

  fn create_test(&self, root: &Path, path: &Path) -> Result<(), Box<dyn Error>> {
    let first_parent = path.iter().next().unwrap();
    let child_path = path.strip_prefix(first_parent).unwrap().parent().unwrap();

    let file_stem = path.file_stem().unwrap().to_str().unwrap();
    let test_file_name = format!("test_{}.py", file_stem);
    let test_folder = root.join(Path::new("tests").join(child_path));
    let test_path = test_folder.join(test_file_name);
    if test_path.exists() {
      bail!(format!(
        "Test file already exists. Run it with `python -m unittest {}`",
        &test_path.to_str().unwrap(),
      ));
    }

    let path_without_extension = path.with_extension("");

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

    println!(
      "Test file created! Run it with `python -m unittest {}`",
      &test_path.to_str().unwrap()
    );
    Ok(())
  }
}
