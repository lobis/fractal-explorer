use crate::Complex;

use image::{ImageBuffer, RgbImage};

pub fn julia_point(
    z: &Complex,
    c: &Complex,
    iteration_limit: Option<u32>,
    z_mag2_limit: Option<f64>,
) -> f64 {
    let z_mag2_limit: f64 = z_mag2_limit.unwrap_or(10.0);
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
    color_function: Option<fn(f64) -> [u8; 3]>,
) -> RgbImage {
    let pixel_size = (
        (range.0 .1 - range.0 .0) / image_size_in_pixel.0 as f64,
        (range.1 .1 - range.1 .0) / image_size_in_pixel.1 as f64,
    );

    let mut image: RgbImage = ImageBuffer::new(image_size_in_pixel.0, image_size_in_pixel.1);
    let color_function = color_function.unwrap_or(|intensity| {
        let pixel_value = (intensity * 255 as f64) as u8;
        return [pixel_value, pixel_value, pixel_value];
    });
    for x in 0..image_size_in_pixel.0 {
        for y in 0..image_size_in_pixel.1 {
            let z = Complex {
                real: range.0 .0 + x as f64 * pixel_size.0 - center.0,
                imaginary: range.1 .0 + y as f64 * pixel_size.1 - center.1,
            };
            let intensity = julia_point_function(z);

            let pixel = image.get_pixel_mut(x, y);
            let image::Rgb(_) = *pixel;
            *pixel = image::Rgb(color_function(intensity));
        }
    }

    return image;
}

// TODO: not working with user submitted colors, arguments unused
pub fn color_interpolation_generator(
    color_start: [u8; 3], // closest to border of fractal
    color_end: [u8; 3],   // outside of fractal
) -> fn(f64) -> [u8; 3] {
    return |intensity: f64| -> [u8; 3] {
        // intensity in range [0.0, 1.0]
        if intensity >= 1.0 {
            return [0, 0, 0];
        }

        // linear interpolation between two colors
        let color_start: [u8; 3] = [255, 0, 0]; // closest to border of fractal
        let color_end: [u8; 3] = [255, 255, 0]; // outside of fractal

        let rgb = [
            (color_end[0] as f64 * intensity + color_start[0] as f64 * (1.0 - intensity)) as u8,
            (color_end[1] as f64 * intensity + color_start[1] as f64 * (1.0 - intensity)) as u8,
            (color_end[2] as f64 * intensity + color_start[2] as f64 * (1.0 - intensity)) as u8,
        ];
        return rgb;
    };
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

    #[test]
    fn test_julia_set_image() {
        let size_pixel = 512;
        let range_half = 1.55;

        fn julia_function(z: Complex) -> f64 {
            julia_point(
                &z,
                &Complex {
                    real: -0.75,
                    imaginary: 0.0,
                },
                Some(25),
                None,
            )
        }

        let color_function = color_interpolation_generator([255, 0, 0], [255, 255, 0]);

        let image = julia_set(
            (size_pixel, size_pixel),
            ((-range_half, range_half), (-range_half, range_half)),
            (0.0, 0.0),
            julia_function,
            Some(color_function),
        );
        image.save("fractal.png").unwrap();
    }
}
