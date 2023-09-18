// &'static est un "lifetime specifier", vous verrez plus tard à quoi cela correspond
pub fn hello(_s1: &str) -> String {
  String::new()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[ignore]
  #[test]
  fn test_hello_with_empty_string_is_compared_by_value() {
    assert_eq!("Hello, World!", hello(""));
  }

  #[ignore]
  #[test]
  fn test_hello_should_return_hello_alice_when_alice() {
    assert_eq!("Hello, Alice!", hello("Alice"));
  }

  #[ignore]
  #[test]
  fn test_hello_should_return_hello_bob_when_bob() {
    assert_eq!("Hello, Bob!", hello("Bob"));
  }
}
