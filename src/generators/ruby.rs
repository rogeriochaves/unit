use super::super::Generator;
use std::error::Error;
use std::fs;
use std::path::Path;
extern crate inflector;
use inflector::Inflector;
use std::env;

extern crate cmd_lib;
use cmd_lib::run_cmd;

pub struct Std();

impl Generator for Std {
  fn option_name(&self) -> &'static str {
    "std"
  }

  fn run_command(&self, test_path: &Path) -> String {
    format!("ruby {}", test_path.display())
  }

  fn create_test(&self, root: &Path, path: &Path) -> Result<(), Box<dyn Error>> {
    let (child_path, file_stem, path_without_extension, levels_up) = self.path_destructing(&path);

    let test_folder = root.join("test").join(child_path);
    let test_file_name = format!("test_{}.rb", file_stem);
    let test_path = test_folder.join(test_file_name);
    self.check_existing(&test_path)?;

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

    self.success_message(&test_path);
    Ok(())
  }
}

pub struct Rspec();

impl Generator for Rspec {
  fn option_name(&self) -> &'static str {
    "rspec"
  }

  fn run_command(&self, test_path: &Path) -> String {
    format!("bin/rspec {}", test_path.display())
  }

  fn setup(&self, root: &Path) -> Result<(), Box<dyn Error>> {
    if root.join("spec/spec_helper.rb").exists() {
      return Ok(());
    }

    let gemfile_path = root.join("Gemfile");
    let mut gemfile = fs::read_to_string(&gemfile_path).or(Err(
      "Gemfile not found, it is necessary for installing rspec",
    ))?;
    if !gemfile.contains("rspec") {
      gemfile = gemfile + "\n\ngem 'rspec', '~> 3.0'\n";
      fs::write(&gemfile_path, gemfile)?;
    }

    let current_dir = env::current_dir()?;
    if root.to_str().unwrap() != "" {
      env::set_current_dir(root.to_str().unwrap())?;
    }
    run_cmd!("/usr/bin/bundle install --binstubs").unwrap();
    run_cmd!("bin/rspec --init").unwrap();
    env::set_current_dir(current_dir)?;

    Ok(())
  }

  fn create_test(&self, root: &Path, path: &Path) -> Result<(), Box<dyn Error>> {
    self.setup(root)?;

    let (child_path, file_stem, path_without_extension, levels_up) = self.path_destructing(&path);

    let test_folder = root.join("spec").join(child_path);
    let test_file_name = format!("{}_spec.rb", file_stem);
    let test_path = test_folder.join(test_file_name);
    self.check_existing(&test_path)?;

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

    self.success_message(&test_path);
    Ok(())
  }
}
