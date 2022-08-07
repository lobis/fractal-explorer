use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Mul},
};

#[derive(Debug, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub imaginary: f64,
}

impl Add for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            real: self.real * other.real,
            imaginary: self.imaginary * other.imaginary,
        }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut sign: char = '+';
        if self.imaginary < 0.0 {
            sign = '-';
        }
        write!(f, "{} {} i{}", self.real, sign, self.imaginary.abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Note this useful idiom: importing names from outer (for mod tests) scope.

    #[test]
    fn test_addition() {
        assert_eq!(
            Complex {
                real: 1.0,
                imaginary: -2.0
            } + Complex {
                real: -3.5,
                imaginary: 4.0
            },
            Complex {
                real: -2.5,
                imaginary: 2.0
            }
        );
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(
            Complex {
                real: 1.25,
                imaginary: 0.0
            } * Complex {
                real: -1.5,
                imaginary: 2.0
            },
            Complex {
                real: -1.875,
                imaginary: 0.0
            }
        );
    }

    #[test]
    fn test_print() {
        println!(
            "{}",
            Complex {
                real: 1.5,
                imaginary: -2.0
            }
        )
    }
}
