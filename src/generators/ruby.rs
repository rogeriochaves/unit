use super::Generator;
use std::error::Error;
use std::fs;
use std::path::Path;
extern crate inflector;
use inflector::Inflector;

pub struct Std();

impl Generator for Std {
  fn create_test(&self, root: &Path, path: &Path) -> Result<(), Box<dyn Error>> {
    let first_parent = path.iter().next().unwrap();
    let child_path = path.strip_prefix(first_parent).unwrap().parent().unwrap();

    let file_stem = path.file_stem().unwrap().to_str().unwrap();
    let test_file_name = format!("test_{}.rb", file_stem);
    let test_folder = root.join(Path::new("test").join(child_path));
    let test_path = test_folder.join(test_file_name);
    println!("test_path: {:?}", test_path);

    let levels_up = vec!["../"; child_path.components().count() + 1].join("");
    let path_without_extension = path.with_extension("");

    fs::create_dir_all(test_folder).unwrap_or_default();
    fs::write(
      test_path,
      format!(
        r#"require_relative "{}{}"
require "test/unit"

class Test{} < Test::Unit::TestCase
  def test_it_works
    assert_equal(4, 2 + 2)
  end
end
"#,
        levels_up,
        path_without_extension.to_str().unwrap(),
        file_stem.to_class_case()
      ),
    )?;

    Ok(())
  }

  fn test_is_present(&self, _path: &Path) -> Result<bool, Box<dyn Error>> {
    Ok(false)
    // let content = fs::read_to_string(path)?;
    // Ok(content.contains("#[test]"))
  }
}
