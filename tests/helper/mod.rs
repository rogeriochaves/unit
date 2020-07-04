use std::fs;
use std::path::{Path, PathBuf};

extern crate fs_extra;
extern crate rand;

use rand::distributions::Alphanumeric;
use rand::Rng;

struct Cleaning {
  state: u16,
}
impl Cleaning {
  fn done(&mut self) -> bool {
    if self.state == 0 {
      self.state = 1;
      fs_extra::remove_items(&vec!["tests/.examples.tmp"]).unwrap_or_default();
      self.state = 2;
    }
    return self.state == 2;
  }
}
static mut EXAMPLE_TMP_CLEANING: Cleaning = Cleaning { state: 0 };

pub fn get_examples_path() -> PathBuf {
  // Workaround to be sure to clean the folder before the first run, to preven
  // keep creating new folders forever
  while unsafe { !EXAMPLE_TMP_CLEANING.done() } {}

  fs::create_dir("tests/.examples.tmp").unwrap_or_default();

  let hash = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(10)
    .collect::<String>();

  let target = format!("tests/.examples.tmp/{}", hash);
  let mut options = fs_extra::dir::CopyOptions::new();
  options.copy_inside = true;
  options.overwrite = true;
  fs_extra::copy_items(&vec!["tests/examples"], &target, &options).unwrap();

  PathBuf::from(target)
}

pub fn compare_files(left_path: &Path, right_path: &Path) {
  let left = fs::read_to_string(&left_path)
    .map_err(|err| format!("Error reading file {:?}: {}", &left_path, err))
    .unwrap();
  let right = fs::read_to_string(&right_path)
    .map_err(|err| format!("Error reading file {:?}: {}", &right_path, err))
    .unwrap();

  if left == right {
    assert_eq!(left, right);
  } else {
    eprintln!(
      r#"
left:

{}

right:

{}
    "#,
      left, right
    );
    panic!("compare files failed");
  }
}
