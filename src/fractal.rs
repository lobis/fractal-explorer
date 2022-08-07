use crate::Complex;

use image::{ImageBuffer, RgbImage};

pub fn julia_point(
    z: &Complex,
    c: &Complex,
    iteration_limit: Option<u32>,
    z_mag2_limit: Option<f64>,
) -> f64 {
    let z_mag2_limit: f64 = z_mag2_limit.unwrap_or(100.0);
    let iteration_limit: u32 = iteration_limit.unwrap_or(255);

    let mut z = z.clone();
    let c = c.clone();
    let mut iteration: u32 = 0;
    while z.mag2() < z_mag2_limit && iteration < iteration_limit {
        iteration += 1;
        z = z * z + c;
    }

    return iteration as f64 / iteration_limit as f64;
}

pub fn julia_set(
    image_size_in_pixel: (u32, u32),
    range: ((f64, f64), (f64, f64)),
    center: (f64, f64),
    julia_point_function: fn(Complex) -> f64,
) -> RgbImage {
    let pixel_size = (
        (range.0 .1 - range.0 .0) / image_size_in_pixel.0 as f64,
        (range.1 .1 - range.1 .0) / image_size_in_pixel.1 as f64,
    );

    let mut image: RgbImage = ImageBuffer::new(image_size_in_pixel.0, image_size_in_pixel.1);

    for x in 0..image_size_in_pixel.0 {
        for y in 0..image_size_in_pixel.1 {
            let z = Complex {
                real: range.0 .0 + x as f64 * pixel_size.0 - center.0,
                imaginary: range.1 .0 + y as f64 * pixel_size.1 - center.1,
            };
            let intensity = julia_point_function(z);
            let color_intensity = (intensity * 255 as f64) as u8;

            let pixel = image.get_pixel_mut(x, y);
            let image::Rgb(_) = *pixel;
            *pixel = image::Rgb([color_intensity, color_intensity, color_intensity]);
        }
    }

    return image;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_julia_point() {
        let z_initial = Complex {
            real: 0.0,
            imaginary: 0.0,
        };
        let offset = Complex {
            real: 0.0,
            imaginary: 0.0,
        };

        assert_eq!(julia_point(&z_initial, &offset, None, None), 1.0);
    }
}
