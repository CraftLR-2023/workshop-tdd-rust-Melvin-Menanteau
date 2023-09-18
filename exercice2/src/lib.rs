pub fn compute_string(_i: usize) -> String {
  String::new()
}

pub fn compute_list(_i: usize) -> Vec<String> {
  vec!()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[ignore]
  #[test]
  fn test_compute_string_should_return_1_when_1() {
    let result = compute_string(1);
    assert_eq!("1", result);
  }

  #[ignore]
  #[test]
  fn test_compute_string_should_return_2_when_2() {
    let result = compute_string(2);
    assert_eq!("2", result);
  }

  #[ignore]
  #[test]
  fn test_compute_string_should_return_fizz_when_3() {
    let result = compute_string(3);
    assert_eq!("Fizz", result);
  }

  #[ignore]
  #[test]
  fn test_compute_string_should_return_4_when_4() {
    let result = compute_string(4);
    assert_eq!("4", result);
  }

  #[ignore]
  #[test]
  fn test_compute_string_should_return_buzz_when_5() {
    let result = compute_string(5);
    assert_eq!("Buzz", result);
  }

  #[ignore]
  #[test]
  fn test_compute_string_should_return_fizz_when_6() {
    let result = compute_string(6);
    assert_eq!("Fizz", result);
  }

  #[ignore]
  #[test]
  fn test_compute_string_should_return_fizz_when_9() {
    let result = compute_string(9);
    assert_eq!("Fizz", result);
  }

  #[ignore]
  #[test]
  fn test_compute_string_should_return_buzz_when_10() {
    let result = compute_string(10);
    assert_eq!("Buzz", result);
  }

  #[ignore]
  #[test]
  fn test_compute_string_should_return_fizz_buzz_when_15() {
    let result = compute_string(15);
    assert_eq!("FizzBuzz", result);
  }

  #[ignore]
  #[test]
  fn test_compute_list_should_return_a_sequence_of_5_elements_when_5() {
    let result = compute_list(5);
    assert_eq!(result.len(), 5)
  }

  #[ignore]
  #[test]
  fn test_compute_list_should_return_a_sequence_of_5_fizz_buzz_elements_when_5() {
    let result = compute_list(5);
    assert_eq!(result, vec!("1", "2", "Fizz", "4", "Buzz"));
  }

  #[ignore]
  #[test]
  fn test_compute_list_should_return_a_sequence_of_15_fizz_buzz_elements_when_15() {
    let result = compute_list(15);
    for i in 0..result.len() {
      assert_eq!(compute_string(i + 1), result[i])
    }
  }
}
