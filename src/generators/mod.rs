use std::error::Error;
use std::path::Path;
pub mod ruby;
pub mod rust;

pub trait Generator {
  fn create_test(&self, root: &Path, path: &Path) -> Result<(), Box<dyn Error>>;
  fn test_is_present(&self, path: &Path) -> Result<bool, Box<dyn Error>>;
}
