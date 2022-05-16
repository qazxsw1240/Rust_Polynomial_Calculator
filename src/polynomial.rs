use std::ops::{Add, Sub};
use std::str::FromStr;

use crate::Monomial;

trait SubStr {
    fn substr(&self, start: usize, len: usize) -> String;
}

impl SubStr for String {
    fn substr(&self, start: usize, len: usize) -> String {
        let mut res = String::new();

        for (i, ch) in self.chars().skip(start).enumerate() {
            if i == len {
                break;
            }

            res.push(ch);
        }

        res
    }
}

#[derive(Clone, Debug)]
pub struct Polynomial {
    monomials: Vec<Monomial>,
}

impl Polynomial {
    pub fn new() -> Self {
        Self {
            monomials: Vec::new()
        }
    }

    pub fn push(&mut self, m: Monomial) {
        for (i, monomial) in self.monomials.iter_mut().enumerate() {
            if monomial.power == m.power {
                monomial.coefficient += m.coefficient;

                if monomial.coefficient == 0 {
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

        for (i, &m) in self.monomials.iter().enumerate() {
            if i != 0 {
                if m.coefficient < 0 {
                    res.push_str(" - ");
                } else {
                    res.push_str(" + ");
                }
            }

            if i == 0 {
                res.push_str(m.to_string().as_str())
            } else {
                let abs_monomial = Monomial::new(m.coefficient.abs(), m.power);

                res.push_str(abs_monomial.to_string().as_str())
            }
        }

        res
    }
}

impl FromStr for Polynomial {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut compressed_string = String::from(s);

        compressed_string = compressed_string.replace(" ", "");

        if compressed_string.len() == 0 {
            return Err(());
        }

        for &byte in compressed_string.as_bytes() {
            let ch = byte as char;

            if !ch.is_digit(10) && ch != '+' && ch != '-' && ch != '^' && ch != 'x' {
                return Err(());
            }
        }

        if (compressed_string.as_bytes()[0] as char).is_digit(10) || (compressed_string.as_bytes()[0] as char) == 'x' {
            compressed_string.insert(0, '+');
        }

        compressed_string.push('+');

        let mut j = 0usize;
        let mut monomial_strings = Vec::<String>::new();

        let bytes = compressed_string.as_bytes();

        for (i, &byte) in bytes.iter().enumerate().skip(1) {
            let ch = byte as char;

            if ch != '+' && ch != '-' || (i != 0 && (bytes[i - 1] as char) == '^') {
                continue;
            }

            monomial_strings.push(compressed_string.substr(j, i - j));
            j = i;
        }

        for monomial_string in &mut monomial_strings {
            match monomial_string.chars().nth(1) {
                None => {
                    return Err(());
                }
                Some(ch) => {
                    if ch == 'x' {
                        monomial_string.insert(1, '1');
                    }
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
            match Monomial::from_str(monomial_string.as_str()) {
                Ok(m) => {
                    polynomial.push(m);
                }
                Err(_) => {
                    return Err(());
                }
            }
        }

        Ok(polynomial)
    }
}

impl Add for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self.clone();

        for &m in &rhs.monomials {
            res.push(m);
        }

        res
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = self.clone();

        for &m in &rhs.monomials {
            res.push(-m);
        }

        res
    }
}