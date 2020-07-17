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

  fn create_test(&self, root: &Path, path: &Path) -> Result<(), Box<dyn Error>> {
    let path = root.join(path);
    let mut content = fs::read_to_string(&path)?;

    if content.contains("#[test]") {
      bail!(format!(
        "{} has already tests on it. Run it with `cargo test`",
        &path.to_str().unwrap()
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
      "Tests added to file {}. Run it with `cargo test`",
      &path.to_str().unwrap()
    );
    Ok(())
  }
}
