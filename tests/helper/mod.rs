use std::fs;
use std::path::{Path, PathBuf};

extern crate fs_extra;
extern crate rand;

use rand::distributions::Alphanumeric;
use rand::Rng;
use std::env;

struct Cleaning {
  state: u16,
  current_dir: Option<PathBuf>,
}
impl Cleaning {
  fn done(&mut self) -> bool {
    if self.current_dir.is_none() {
      self.current_dir = Some(env::current_dir().unwrap());
    }
    if self.state == 0 {
      self.state = 1;
      fs_extra::remove_items(&vec!["tests/.examples.tmp"]).unwrap_or_default();
      self.state = 2;
    }
    return self.state == 2;
  }
}
static mut EXAMPLE_TMP_CLEANING: Cleaning = Cleaning {
  state: 0,
  current_dir: None,
};

pub fn get_examples_path() -> PathBuf {
  // Workaround to be sure to clean the folder before the first run, to preven
  // keep creating new folders forever
  while unsafe { !EXAMPLE_TMP_CLEANING.done() } {}
  let current_dir = unsafe { &EXAMPLE_TMP_CLEANING.current_dir };
  let current_dir = current_dir.as_ref().unwrap();

  fs::create_dir(current_dir.join("tests/.examples.tmp")).unwrap_or_default();

  let hash = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(10)
    .collect::<String>();

  let target = current_dir.join("tests/.examples.tmp").join(hash);
  let mut options = fs_extra::dir::CopyOptions::new();
  options.copy_inside = true;
  options.overwrite = true;
  let sources = &vec![current_dir.join("tests/examples")];
  fs_extra::copy_items(&sources, &target, &options).unwrap();

  current_dir.join(target)
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
