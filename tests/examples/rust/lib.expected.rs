pub fn foo() {
  println!("bar")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(2, 1 + 1);
  }
}
