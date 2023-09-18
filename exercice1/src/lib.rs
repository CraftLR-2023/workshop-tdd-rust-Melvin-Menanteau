// &'static est un "lifetime specifier", vous verrez plus tard à quoi cela correspond
pub fn hello(_s1: &str) -> String {
    if !_s1.is_empty() {
        format!("Hello, {}!", _s1)
    } else {
        String::from("Hello, World!")
    }
}

#[cfg(test)]
mod tests {
  use super::*;


  #[test]
  fn test_hello_with_empty_string_is_compared_by_value() {
    assert_eq!("Hello, World!", hello(""));
  }

  
  #[test]
  fn test_hello_should_return_hello_alice_when_alice() {
    assert_eq!("Hello, Alice!", hello("Alice"));
  }

  #[test]
  fn test_hello_should_return_hello_bob_when_bob() {
    assert_eq!("Hello, Bob!", hello("Bob"));
  }
}