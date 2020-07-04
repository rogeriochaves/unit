use std::error::Error;
use std::fs;
use std::path::Path;

pub fn create_test(path: &Path) -> Result<(), Box<dyn Error>> {
  let mut content = fs::read_to_string(path)?;

  content = format!(
    r#"{}
#[cfg(test)]
mod tests {{
  use super::*;

  #[test]
  fn it_works() {{
    assert_eq!(2, 1 + 1);
  }}
}}
"#,
    content
  );

  fs::write(path, content)?;

  Ok(())
}

pub fn test_is_present(path: &Path) -> Result<bool, Box<dyn Error>> {
  let content = fs::read_to_string(path)?;
  Ok(content.contains("#[test]"))
}

pub fn run(path: &Path) -> Result<(), Box<dyn Error>> {
  if test_is_present(path)? {
    println!(
      "Test already created for {}. Run it with `cargo test`",
      path.to_str().unwrap()
    );
  } else {
    create_test(path)?;
    println!("Done! Run test with `cargo test`");
  }

  Ok(())
}
