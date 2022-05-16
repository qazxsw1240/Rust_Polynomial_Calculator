use std::cmp::Ordering;
use std::ops::Neg;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub struct Monomial {
    pub coefficient: i32,
    pub power: i32,
}

impl Monomial {
    pub fn new(coefficient: i32, power: i32) -> Self {
        Self {
            coefficient,
            power,
        }
    }

    pub fn to_string(&self) -> String {
        let mut res = String::new();

        if self.coefficient < 0 {
            res.push('-');
        }

        if self.coefficient.abs() != 1 {
            res += self.coefficient.abs().to_string().as_str();
        }

        if self.power == 0 {
            res.clone()
        } else {
            res.push('x');

            if self.power != 1 {
                res.push('^');
                res += self.power.to_string().as_str();
            }

            res.clone()
        }
    }
}

impl FromStr for Monomial {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string = String::from(s);
        let str_split = string.split("x^").collect::<Vec<&str>>();

        if str_split.len() != 2 {
            Err(())
        } else {
            let coefficient = i32::from_str(str_split[0]).expect("Invalid format provided.");
            let power = i32::from_str(str_split[1]).expect("Invalid format provided.");

            Ok(Monomial::new(coefficient, power))
        }
    }
}

impl Neg for Monomial {
    type Output = Monomial;

    fn neg(self) -> Self::Output {
        Monomial::new(-self.coefficient, self.power)
    }
}


impl PartialEq<Self> for Monomial {
    fn eq(&self, other: &Self) -> bool {
        self.coefficient == other.coefficient && self.power == other.power
    }
}

impl PartialOrd<Self> for Monomial {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match other.power.partial_cmp(&self.power) {
            None => None,
            Some(ord) => match ord {
                Ordering::Equal => self.coefficient.partial_cmp(&other.coefficient),
                other => Some(other)
            },
        }
    }
}

impl Eq for Monomial {}

impl Ord for Monomial {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.power.cmp(&self.power) {
            Ordering::Equal => self.coefficient.cmp(&other.coefficient),
            other => other
        }
    }
}