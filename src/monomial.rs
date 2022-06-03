use std::cmp::Ordering;
use std::ops::Neg;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub struct Monomial {
    coefficient: i32,
    power: i32,
}

impl Monomial {
    pub fn new(coefficient: i32, power: i32) -> Self {
        Self { coefficient, power }
    }

    pub fn is_zero(&self) -> bool {
        self.coefficient == 0
    }

    pub fn is_same_power(&self, other: &Self) -> bool {
        self.power == other.power
    }

    pub fn add_coefficient(&mut self, other: &Self) {
        self.coefficient += other.coefficient;
    }

    pub fn abs(&self) -> Self {
        Self {
            coefficient: self.coefficient.abs(),
            power: self.power,
        }
    }

    pub fn sgn(&self) -> i8 {
        if self.coefficient > 0 {
            1
        } else if self.coefficient < 0 {
            -1
        } else {
            0
        }
    }

    pub fn to_string(&self) -> String {
        let mut res = String::new();

        if self.coefficient < 0 {
            res.push('-');
        }

        if self.coefficient.abs() != 1 || self.power == 0 {
            res.push_str(self.coefficient.abs().to_string().as_str());
        }

        if self.power != 0 {
            res.push('x');

            if self.power != 1 {
                res.push('^');
                res.push_str(self.power.to_string().as_str());
            }
        }

        res
    }
}

impl FromStr for Monomial {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split("x^").collect::<Vec<&str>>();

        if split.len() != 2 {
            Err(())
        } else {
            let coefficient = i32::from_str(split[0]).expect("Invalid format provided.");
            let power = i32::from_str(split[1]).expect("Invalid format provided.");

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
                other => Some(other),
            },
        }
    }
}

impl Eq for Monomial {}

impl Ord for Monomial {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.power.cmp(&self.power) {
            Ordering::Equal => self.coefficient.cmp(&other.coefficient),
            other => other,
        }
    }
}
