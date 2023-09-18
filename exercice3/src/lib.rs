fn convert(c: char) -> u32 {
	match c {
		'I' => 1,
		'V' => 5,
		'X' => 10,
		'L' => 50,
		'C' => 100,
		'D' => 500,
		'M' => 1000,
		_ => 0
	}
}

pub fn en_nombre_arabe(_nombre_romain: &str) -> u32 {
	let mut r = 0;
	
	for (index, number) in _nombre_romain.char_indices() {
		if index > 0 {
			r += convert(number);

			if convert(_nombre_romain.chars().nth(index - 1).unwrap()) < convert(number) {
				r -= convert(_nombre_romain.chars().nth(index - 1).unwrap()) * 2;
			}
		} else {
			r += convert(number);
		}
		println!("{}", r);
	}

	r
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn le_chiffre_romain_i_devrait_donner_1() {
    let nombre = en_nombre_arabe("I");
    assert_eq!(1, nombre);
  }

  #[test]
  fn le_chiffre_romain_ii_devrait_donner_2() {
    let nombre = en_nombre_arabe("II");
    assert_eq!(2, nombre);
  }

  #[test]
  fn le_chiffre_romain_iii_devrait_donner_3() {
    let nombre = en_nombre_arabe("III");
    assert_eq!(3, nombre);
  }

  #[test]
  fn le_chiffre_romain_v_devrait_donner_5() {
    let nombre = en_nombre_arabe("V");
    assert_eq!(5, nombre);
  }

  #[test]
  fn le_chiffre_romain_vi_devrait_donner_6() {
    let nombre = en_nombre_arabe("VI");
    assert_eq!(6, nombre);
  }

  #[test]
  fn le_chiffre_romain_iv_devrait_donner_4() {
    let nombre = en_nombre_arabe("IV");
    assert_eq!(4, nombre);
  }

  #[test]
  fn le_chiffre_romain_x_devrait_donner_10() {
    let nombre = en_nombre_arabe("X");
    assert_eq!(10, nombre);
  }

  #[test]
  fn le_chiffre_romain_xi_devrait_donner_11() {
    let nombre = en_nombre_arabe("XI");
    assert_eq!(11, nombre);
  }

  #[test]
  fn le_chiffre_romain_l_devrait_donner_50() {
    let nombre = en_nombre_arabe("L");
    assert_eq!(50, nombre);
  }

  #[test]
  fn le_chiffre_romain_xlix_devrait_donner_49() {
    let nombre = en_nombre_arabe("XLIX");
    assert_eq!(49, nombre);
  }

  #[test]
  fn le_chiffre_romain_c_devrait_donner_100() {
    let nombre = en_nombre_arabe("C");
    assert_eq!(100, nombre);
  }

  #[test]
  fn le_chiffre_romain_xcix_devrait_donner_99() {
    let nombre = en_nombre_arabe("XCIX");
    assert_eq!(99, nombre);
  }

  #[test]
  fn le_chiffre_romain_cccxlvi_devrait_donner_346() {
    let nombre = en_nombre_arabe("CCCXLVI");
    assert_eq!(346, nombre);
  }

  #[test]
  fn le_chiffre_romain_cccxcix_devrait_donner_399() {
    let nombre = en_nombre_arabe("CCCXCIX");
    assert_eq!(399, nombre);
  }

  #[test]
  fn le_chiffre_romain_d_devrait_donner_500() {
    let nombre = en_nombre_arabe("D");
    assert_eq!(500, nombre);
  }

  #[test]
  fn le_chiffre_romain_m_devrait_donner_1000() {
    let nombre = en_nombre_arabe("M");
    assert_eq!(1000, nombre);
  }

  #[test]
  fn le_chiffre_romain_mmmmdccclxxxviii_devrait_donner_4888() {
    let nombre = en_nombre_arabe("MMMMDCCCLXXXVIII");
    assert_eq!(4888, nombre);
  }
}
