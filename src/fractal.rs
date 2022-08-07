use crate::Complex;

pub fn julia(z_initial: Complex, offset: Complex) -> u32 {
    let z_mag2_max: f64 = 100.0;
    let iteration_max: u32 = 100;

    let mut z = z_initial;
    let mut iteration: u32 = 0;

    while z.mag2() < z_mag2_max && iteration < iteration_max {
        iteration += 1;
        z = z * z + offset;
    }

    return iteration;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_julia() {
        let z_initial = Complex {
            real: 0.0,
            imaginary: 0.0,
        };
        let offset = Complex {
            real: 0.0,
            imaginary: 0.0,
        };

        assert_eq!(julia(z_initial, offset), 100);
    }
}
