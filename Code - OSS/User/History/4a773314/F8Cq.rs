use std::vec;

fn main() {
    let big_val = std::i32::MAX;
    // let x = big_val + 1;
    let x = big_val.wrapping_add(1);
    println!("x = {x}");
    let y = (std::i32::MAX as i64 + 1 as i64) as i64;
    println!("y = {y}");

    let t = (12, "eggs");
    let b = Box::new(t);

    let languages: Vec<String> = std::env::args().skip(1).collect();
    for i in languages {
        println!("{}: {}", 1
            if i.len() % 2 == 0 {
                "Функциональный"
            } else {
                "Императивный"
            }
        );
    }
}

fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}

#[test]
fn test_convert_types() {
    assert_eq!(10_i8 as u16, 10_u16);
    assert_eq!(2525_u16 as i16, 2525_i16);
    assert_eq!(-1_i16 as i32, -1_i32);
    assert_eq!(65535 as i32, 65535_i32);

    assert_eq!(1000_i16 as u8, 232_u8);
    assert_eq!(65535_u32 as i16, -1_i16);
    assert_eq!(-1_i8 as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);

    assert_eq!(2u16.pow(4), 16);
    assert_eq!((-4i32).abs(), 4);
    assert_eq!(0b101101u8.count_ones(), 4);

    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);
    assert_eq!(-1.01f64.floor(), -1.);
    assert!((-1. / std::f32::INFINITY).is_sign_negative());

    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);
}

#[test]
fn test_vec() {
    let lazy_caterer = [1, 2, 4, 7, 11, 16,];
    let taxonomy = ["Animalia", "Artrophoda", "Insecta"];
    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    assert!(sieve[211]);
    assert!(!sieve[9876]);

    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);

    let mut v = vec![2, 3, 5, 7];
    assert_eq!(v.iter().fold(1, |a, b| a * b), 210);
    v.push(11);
    v.push(13);
    assert_eq!(v.iter().fold(1, |a, b| a * b), 30030);

    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);

    let mut v = vec!["a man", "a plan", "a canal", "panama"];
    v.reverse();
    assert_eq!(v, vec!["panama", "a canal", "a plan", "a man"]);

    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v.push(3);
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 4);

    let mut v = vec![10, 20, 30, 40, 50];
    v.insert(3, 35);
    assert_eq!(v, [10, 20, 30, 35, 40, 50]);

    v.remove(1);
    assert_eq!(v, [10, 30, 35, 40, 50]);

    let mut v = vec!["carmen", "miranda"];
    assert_eq!(v.pop(), Some("miranda"));
    assert_eq!(v.pop(), Some("carmen"));
    assert_eq!(v.pop(), None);
}

fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols]
}