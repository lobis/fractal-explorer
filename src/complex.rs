use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Mul},
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Complex {
    pub real: f64,
    pub imaginary: f64,
}

impl Complex {
    pub fn mag2(&self) -> f64 {
        self.real * self.real + self.imaginary * self.imaginary
    }

    pub fn mag(&self) -> f64 {
        f64::sqrt(self.mag2())
    }
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
    use super::*;

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
    fn test_format() {
        assert_eq!(
            format!(
                "{}",
                Complex {
                    real: 1.5,
                    imaginary: -2.25
                }
            ),
            "1.5 - i2.25"
        )
    }

    #[test]
    fn test_magnitude() {
        let z = Complex {
            real: 3.75,
            imaginary: 1.25,
        };
        let z_mag2 = 15.625;
        assert_eq!(z.mag2(), z_mag2);
        assert_eq!(z.mag(), f64::sqrt(z_mag2));
    }
}
