use std::ops::{Add, Sub};
use std::str::FromStr;

use crate::monomial::Monomial;
use crate::substr::SubStr;

#[derive(Clone, Debug)]
pub struct Polynomial {
    monomials: Vec<Monomial>,
}

impl Polynomial {
    pub fn new() -> Self {
        Self {
            monomials: Vec::new(),
        }
    }

    pub fn push(&mut self, m: Monomial) {
        for (i, monomial) in self.monomials.iter_mut().enumerate() {
            if monomial.is_same_power(&m) {
                monomial.add_coefficient(&m);

                if monomial.is_zero() {
                    self.monomials.remove(i);
                }

                return;
            }
        }

        self.monomials.push(m);
        self.monomials.sort();
    }

    pub fn to_string(&self) -> String {
        let mut res = String::new();

        self.monomials.iter().enumerate().for_each(|(i, &m)| {
            let monomial_str = if i == 0 {
                m.to_string()
            } else {
                m.abs().to_string()
            };

            if i != 0 {
                res.push_str(if m.sgn() < 0 { " - " } else { " + " });
            }

            res.push_str(monomial_str.as_str());
        });

        res
    }
}

impl FromStr for Polynomial {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[inline(always)]
        fn is_valid_char(c: u8) -> bool {
            let ch = c as char;
            ch.is_digit(10) || ch == '+' || ch == '-' || ch == '^' || ch == 'x'
        }

        let mut clean_string = String::from(s).replace(" ", "");

        if clean_string.len() == 0 || clean_string
            .as_bytes()
            .iter()
            .any(|&byte| !is_valid_char(byte))
        {
            return Err(());
        }

        let first_char = clean_string.as_bytes()[0] as char;

        if first_char.is_digit(10) || first_char == 'x' {
            clean_string.insert(0, '+');
        }

        clean_string.push('+');

        let mut j = 0usize;
        let mut monomial_strings = Vec::<String>::new();

        let bytes = clean_string.as_bytes();

        for (i, &byte) in bytes.iter().enumerate().skip(1) {
            let ch = byte as char;

            if ch != '+' && ch != '-' || (i != 0 && (bytes[i - 1] as char) == '^') {
                continue;
            }

            monomial_strings.push(clean_string.substr(j, i - j));
            j = i;
        }

        for monomial_string in &mut monomial_strings {
            if let Some(ch) = monomial_string.chars().nth(1) {
                if ch == 'x' {
                    monomial_string.insert(1, '1');
                }
            }

            if !monomial_string.contains('x') {
                monomial_string.push_str("x^0");
                continue;
            }

            if !monomial_string.contains('^') {
                monomial_string.push_str("^1");
            }
        }

        let mut polynomial = Polynomial::new();

        for monomial_string in monomial_strings {
            if let Ok(m) = Monomial::from_str(monomial_string.as_str()) {
                polynomial.push(m);
            }
        }

        Ok(polynomial)
    }
}

impl Add for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self.clone();
        rhs.monomials.iter().for_each(|&m| res.push(m));
        res
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = self.clone();
        rhs.monomials.iter().for_each(|&m| res.push(-m));
        res
    }
}
