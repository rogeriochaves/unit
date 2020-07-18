use std::fs;
use std::path::{Path, PathBuf};

extern crate fs_extra;
extern crate lazy_static;
extern crate rand;

use lazy_static::lazy_static;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::env;
use std::sync::Mutex;

lazy_static! {
  static ref TESTS_EXAMPLES_TMP_CLEANED: Mutex<bool> = Mutex::new(false);
  static ref TESTS_ORIGINAL_DIR: PathBuf = env::current_dir().unwrap();
}

pub fn get_examples_path() -> PathBuf {
  let mut examples_tmp_cleaned = TESTS_EXAMPLES_TMP_CLEANED.lock().unwrap();
  if !*examples_tmp_cleaned {
    fs_extra::remove_items(&vec!["tests/.examples.tmp"]).unwrap_or_default();
    *examples_tmp_cleaned = true;
  }
  let current_dir = &TESTS_ORIGINAL_DIR;

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
