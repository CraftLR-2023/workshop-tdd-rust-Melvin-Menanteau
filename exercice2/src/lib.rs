pub fn compute_string(_i: usize) -> String {
	if _i % 3 == 0 && _i % 5 == 0 {
		String::from("FizzBuzz")
	} else if _i % 3 == 0 {
		String::from("Fizz")
	} else if _i % 5 == 0 {
		String::from("Buzz")
	} else {
		_i.to_string()
	}
}

pub fn compute_list(_i: usize) -> Vec<String> {
  	let mut result: Vec<String> = Vec::new();

	for number in 0.._i {
		result.push(compute_string(number + 1));
	}

	result
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_compute_string_should_return_1_when_1() {
		let result = compute_string(1);
		assert_eq!("1", result);
	}

	#[test]
	fn test_compute_string_should_return_2_when_2() {
		let result = compute_string(2);
		assert_eq!("2", result);
	}

	#[test]
	fn test_compute_string_should_return_fizz_when_3() {
		let result = compute_string(3);
		assert_eq!("Fizz", result);
	}

	#[test]
	fn test_compute_string_should_return_4_when_4() {
		let result = compute_string(4);
		assert_eq!("4", result);
	}

	#[test]
	fn test_compute_string_should_return_buzz_when_5() {
		let result = compute_string(5);
		assert_eq!("Buzz", result);
	}

	#[test]
	fn test_compute_string_should_return_fizz_when_6() {
		let result = compute_string(6);
		assert_eq!("Fizz", result);
	}

	#[test]
	fn test_compute_string_should_return_fizz_when_9() {
		let result = compute_string(9);
		assert_eq!("Fizz", result);
	}

	#[test]
	fn test_compute_string_should_return_buzz_when_10() {
		let result = compute_string(10);
		assert_eq!("Buzz", result);
	}

	#[test]
	fn test_compute_string_should_return_fizz_buzz_when_15() {
		let result = compute_string(15);
		assert_eq!("FizzBuzz", result);
	}

	#[test]
	fn test_compute_list_should_return_a_sequence_of_5_elements_when_5() {
		let result = compute_list(5);
		assert_eq!(result.len(), 5)
	}

	#[test]
	fn test_compute_list_should_return_a_sequence_of_5_fizz_buzz_elements_when_5() {
		let result = compute_list(5);
		assert_eq!(result, vec!("1", "2", "Fizz", "4", "Buzz"));
	}

	#[test]
	fn test_compute_list_should_return_a_sequence_of_15_fizz_buzz_elements_when_15() {
		let result = compute_list(15);
		for i in 0..result.len() {
		assert_eq!(compute_string(i + 1), result[i])
		}
	}
}
