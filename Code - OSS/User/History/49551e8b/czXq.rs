use num::traits::bounds;
use num::Complex;
use std::process::Output;
use std::{process::Command, str::FromStr};

use std::io::Write;

fn main() {
    extern crate crossbeam;
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        writeln!(std::io::stderr(), "Порядок вызова: mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT").unwrap();
        std::process::exit(1);
    }
    let bounds = parse_pair(&args[2], 'x').expect("ошибка при заборе размеров изображения");
    let upper_left = parse_complex(&args[3]).expect("Ошибка при разборе верхнего левого угла");
    let lower_right = parse_complex(&args[4]).expect("Ошибка при разборке нижнего правого угла");
    let mut pixels = vec![0; bounds.0 * bounds.1];
    //render(&mut pixels, bounds, upper_left, lower_right);
    let threads = 8;
    let row_per_band = bounds.1 / threads + 1;
    {
        let bands = pixels.chunks_mut(row_per_band * bounds.0).collect();
        crossbeam::scope(
            
        )
    }
    write_image(&args[1], &pixels, bounds).expect("Ошибка при записи PNG");
}

fn square_loop(mut x: u64) {
    loop {
        x = x * x;
    }
}

// fn square_add_loop(c: f64) {
//     let mut x = 0;
//     loop {
//         x = x * x + c;
//     }
// }

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex(",-0.0625"), None);
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re, //2
        upper_left.im - lower_right.im //2
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64, //-1 + 5 * 25 / 100 == 0.25
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_poin() {
    assert_eq!(
        pixel_to_point(
            (100, 100),
            (25, 75),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: -0.5, im: -0.5 }
    );
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error>{
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
    Ok(())
}
