use super::super::Generator;
use std::error::Error;
use std::fs;
use std::path::Path;
extern crate inflector;
use inflector::Inflector;
use simple_error::bail;
use std::env;

extern crate cmd_lib;
use cmd_lib::run_cmd;

pub struct Std();

impl Generator for Std {
  fn option_name(&self) -> &'static str {
    "std"
  }

  fn create_test(&self, root: &Path, path: &Path) -> Result<(), Box<dyn Error>> {
    let first_parent = path.iter().next().unwrap();
    let child_path = path.strip_prefix(first_parent).unwrap().parent().unwrap();

    let file_stem = path.file_stem().unwrap().to_str().unwrap();
    let test_file_name = format!("test_{}.rb", file_stem);
    let test_folder = root.join(Path::new("test").join(child_path));
    let test_path = test_folder.join(test_file_name);
    if test_path.exists() {
      bail!(format!(
        "Test file already exists. Run it with `ruby {}`",
        &test_path.to_str().unwrap(),
      ));
    }

    let levels_up = vec!["../"; child_path.components().count() + 1].join("");
    let path_without_extension = path.with_extension("");

    fs::create_dir_all(test_folder).unwrap_or_default();
    fs::write(
      &test_path,
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

    println!(
      "Test file created! Run it with `ruby {}`",
      &test_path.to_str().unwrap()
    );
    Ok(())
  }
}

pub struct Rspec();

impl Generator for Rspec {
  fn option_name(&self) -> &'static str {
    "rspec"
  }

  fn setup(&self, root: &Path) -> Result<(), Box<dyn Error>> {
    if root.join("spec/spec_helper.rb").exists() {
      return Ok(());
    }

    let gemfile_path = root.join("Gemfile");
    let mut gemfile = fs::read_to_string(&gemfile_path)?;
    if !gemfile.contains("rspec") {
      gemfile = gemfile + "\n\ngem 'rspec', '~> 3.0'\n";
      fs::write(&gemfile_path, gemfile)?;
    }

    let root_str = root.to_str().unwrap();
    let current_dir = env::current_dir()?;
    let current_dir = current_dir.to_str().unwrap();
    (run_cmd! {
      use root_str, current_dir;

      cd ${root_str}
      /usr/bin/bundle install --binstubs
      bin/rspec --init
      cd ${current_dir}
    })
    .unwrap();

    Ok(())
  }

  fn create_test(&self, root: &Path, path: &Path) -> Result<(), Box<dyn Error>> {
    self.setup(root)?;

    let first_parent = path.iter().next().unwrap();
    let child_path = path.strip_prefix(first_parent).unwrap().parent().unwrap();

    let file_stem = path.file_stem().unwrap().to_str().unwrap();
    let test_file_name = format!("{}_spec.rb", file_stem);
    let test_folder = root.join(Path::new("spec").join(child_path));
    let test_path = test_folder.join(test_file_name);
    if test_path.exists() {
      bail!(format!(
        "Test file already exists. Run it with `bin/rspec {}`",
        &test_path.to_str().unwrap(),
      ));
    }

    let levels_up = vec!["../"; child_path.components().count() + 1].join("");
    let path_without_extension = path.with_extension("");

    fs::create_dir_all(test_folder).unwrap_or_default();
    fs::write(
      &test_path,
      format!(
        r#"require_relative "{}{}"

RSpec.describe "{}" do
  context "example spec" do
    it "works" do
      expect(1 + 1).to eq 2
    end
  end
end
"#,
        levels_up,
        path_without_extension.to_str().unwrap(),
        file_stem.to_class_case()
      ),
    )?;

    println!(
      "Test file created! Run it with `bin/rspec {}`",
      &test_path.to_str().unwrap()
    );
    Ok(())
  }
}
