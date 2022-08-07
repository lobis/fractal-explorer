use fractal_generator::{fractal::julia_point, fractal::julia_set, Complex};

fn main() {
    let size_pixel = 2048;
    let range_half = 1.55;

    fn julia_function(z: Complex) -> f64 {
        julia_point(
            &z,
            &Complex {
                real: -0.75,
                imaginary: 0.0,
            },
            None,
            None,
        )
    }
    let image = julia_set(
        (size_pixel, size_pixel),
        ((-range_half, range_half), (-range_half, range_half)),
        (0.0, 0.0),
        julia_function,
    );
    image.save("fractal.png").unwrap();
}
