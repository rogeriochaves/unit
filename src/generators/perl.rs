use super::super::Generator;
use std::error::Error;
use std::fs;
use std::path::Path;
extern crate inflector;
use simple_error::bail;
extern crate colored;
use colored::*;

pub struct Std();

impl Generator for Std {
  fn option_name(&self) -> &'static str {
    "std"
  }

  fn run_command(&self, test_path: &Path) -> String {
    format!("prove -Ilib {}", &test_path.display())
  }

  fn success_message(&self, test_path: &Path) {
    println!(
      "{}",
      format!(
        "Test file created! Run it now:\n\n{}\n\nAlso, rename the test file to match your current test case",
        self.run_command(test_path)
      )
      .green()
    );
  }

  fn create_test(&self, root: &Path, path: &Path) -> Result<(), Box<dyn Error>> {
    let (child_path, file_stem, _, _) = self.path_destructing(&path);

    let first_parent = path.iter().next().unwrap();
    if first_parent != "lib" {
      bail!("Unit can only create tests for perl modules inside the ./lib/ folder");
    }

    let test_folder = root.join("t").join(child_path);
    let test_file_name = format!("{}_somecase.t", file_stem.to_lowercase());
    let test_path = test_folder.join(test_file_name);
    self.bail_if_existing(&test_path)?;

    let perl_module = child_path
      .join(&file_stem)
      .to_str()
      .unwrap()
      .replace("/", "::");
    fs::create_dir_all(test_folder).unwrap_or_default();
    fs::write(
      &test_path,
      format!(
        r#"use Test::More;

require_ok('{}');

is(1 + 1, 2, 'it works');

done_testing();
"#,
        perl_module,
      ),
    )?;

    self.success_message(&test_path);
    Ok(())
  }
}
