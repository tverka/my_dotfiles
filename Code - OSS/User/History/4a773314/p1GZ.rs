fn main() {
    let big_val = std::i32::MAX;
    // let x = big_val + 1;
    let x = big_val.wrapping_add(1);
    println!("x = {x}");
    let y = (std::i32::MAX as i64 + 1 as i64) as i64;
    println!("y = {y}");

    let t = (12, "eggs");
    let b = Box::new(t);
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
                j += 1;
            }
        }
    }

}