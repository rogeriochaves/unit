use super::Generator;
use std::error::Error;
use std::fs;
use std::path::Path;

pub struct Std();

impl Generator for Std {
  fn create_test(&self, root: &Path, path: &Path) -> Result<(), Box<dyn Error>> {
    let path = root.join(path);
    let mut content = fs::read_to_string(&path)?;

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

    fs::write(path, content)?;

    Ok(())
  }

  fn test_is_present(&self, path: &Path) -> Result<bool, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    Ok(content.contains("#[test]"))
  }
}
