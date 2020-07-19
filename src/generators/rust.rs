use super::Generator;
use simple_error::bail;
use std::error::Error;
use std::fs;
use std::path::Path;

pub struct Std();

impl Generator for Std {
  fn option_name(&self) -> &'static str {
    "std"
  }

  fn run_command(&self, _test_path: &Path) -> String {
    String::from("cargo test")
  }

  fn create_test(&self, root: &Path, path: &Path) -> Result<(), Box<dyn Error>> {
    let path = root.join(path);
    let mut content = fs::read_to_string(&path)?;

    if content.contains("#[test]") {
      bail!(format!(
        "File already has tests on it. Run it with `{}`",
        self.run_command(&path)
      ));
    }

    content = format!(
      r#"{}
#[cfg(test)]
mod tests {{
  use super::*;

  #[test]
  fn it_works() {{
    assert_eq!(4, 2 + 2);
  }}
}}
"#,
      content
    );

    fs::write(&path, content)?;

    println!(
      "Tests added to file. Run it with `{}`",
      self.run_command(&path)
    );
    Ok(())
  }
}

pub struct Integration();

impl Generator for Integration {
  fn option_name(&self) -> &'static str {
    "integration"
  }

  fn run_command(&self, _test_path: &Path) -> String {
    String::from("cargo test")
  }

  fn create_test(&self, root: &Path, path: &Path) -> Result<(), Box<dyn Error>> {
    let (child_path, file_stem, _, _) = self.path_destructing(&path);

    let test_folder = root.join("tests").join(child_path);
    let test_file_name = format!("{}_test.rs", file_stem);
    let test_path = test_folder.join(test_file_name);
    self.bail_if_existing(&test_path)?;

    let rust_module = child_path
      .join(&file_stem)
      .to_str()
      .unwrap()
      .replace("/", "::");
    fs::create_dir_all(test_folder).unwrap_or_default();
    fs::write(
      &test_path,
      format!(
        r#"use {};

#[test]
fn it_works() {{
  assert_eq!(1 + 1, 2);
}}
"#,
        rust_module,
      ),
    )?;

    self.success_message(&test_path);
    Ok(())
  }
}
