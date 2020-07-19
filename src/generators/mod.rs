use std::error::Error;
use std::path::{Path, PathBuf};
pub mod elm;
pub mod javascript;
pub mod perl;
pub mod python;
pub mod ruby;
pub mod rust;
use simple_error::bail;
extern crate colored;
use colored::*;

pub trait Generator {
  fn option_name(&self) -> &'static str;
  fn run_command(&self, test_path: &Path) -> String;
  fn create_test(&self, root: &Path, path: &Path) -> Result<(), Box<dyn Error>>;
  fn setup(&self, _root: &Path) -> Result<(), Box<dyn Error>> {
    Ok(())
  }
  fn bail_if_existing(&self, test_path: &Path) -> Result<(), Box<dyn Error>> {
    if test_path.exists() {
      bail!(format!(
        "Test file already exists. Run it with:\n\n{}\n",
        self.run_command(test_path),
      ));
    }
    Ok(())
  }
  fn success_message(&self, test_path: &Path) {
    println!(
      "{}",
      format!(
        "Test file created! Run it now:\n\n{}\n",
        self.run_command(test_path)
      )
      .green()
    );
  }
  fn path_destructing<'a>(&self, path: &'a Path) -> (&'a Path, &'a str, PathBuf, String) {
    let first_parent = path.iter().next().unwrap();
    let child_path = path.strip_prefix(first_parent).unwrap().parent().unwrap();

    let file_stem = path.file_stem().unwrap().to_str().unwrap();
    let path_without_extension = path.with_extension("");
    let levels_up = vec!["../"; child_path.components().count() + 1].join("");

    return (child_path, file_stem, path_without_extension, levels_up);
  }
}
