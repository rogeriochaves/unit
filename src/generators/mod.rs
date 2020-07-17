use std::error::Error;
use std::path::Path;
pub mod ruby;
pub mod rust;

pub trait Generator {
  fn option_name(&self) -> &'static str;
  fn create_test(&self, root: &Path, path: &Path) -> Result<(), Box<dyn Error>>;
  fn setup(&self, _root: &Path) -> Result<(), Box<dyn Error>> {
    Ok(())
  }
}
