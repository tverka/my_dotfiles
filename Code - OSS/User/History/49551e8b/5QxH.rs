use num::Complex;
use std::{process::Command, str::FromStr};

fn main() {
    println!("Hello, world!");
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
        upper_left.im - lower_right.im, //2
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64, //-1 + 5 * 25 / 100 == 0.25
        im: upper_left.im + pixel.1 as f64 * height / bounds.1 as f64,
    }

}

#[test]
fn test_pixel_to_poin() {
    assert_eq!(pixel_to_point((100, 100), (25, 75), Complex { re: -1.0, im: 1.0 }, Complex { re: 1.0, im: -1.0 }), Complex { re: -0.5, im: -0.5 });
}

fn render(
    pixels: &[u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>
) {
    assert!(pixels.len() == bounds.0 * bounds.1);
    for row in 0..
}