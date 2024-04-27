use std::vec;

fn main() {
    let s = vec!["udon".to_string(), "ramne".to_string(), "soba".to_string()];
    let t = s.clone();
    let u = s.clone();

    let mut s = "Govinda".to_string();
    s = "Siddhartha".to_string();

    let mut s = "Govinda".to_string();
    let t = s;
    s = "Siddhartha".to_string();

    struct Person {
        name: String,
        birth: i32,
    }

    let mut composers = Vec::new();
    composers.push(Person { name: "Palestrina".to_string(), birth: 1525 });

    let x = vec![10, 20, 30];
    let c = true;
    if c {
        f(x);
    } else {
        g(x);
    }
    // h(x);

    let x = vec![10, 20, 30];
    while true {
        g(x);
        x = h(x);
    }
}

fn f(x: Vec<i32>) {

}

fn g(x: Vec<i32>) {

}

fn h(x: Vec<i32>) {

}