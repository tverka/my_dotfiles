use std::{mem, vec};

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
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });

    let x = vec![10, 20, 30];
    let c = true;
    if c {
        f(x);
    } else {
        g(x);
    }
    // h(x);

    let x = vec![10, 20, 30];
    // while true {
    // g(x);
    // x = h(x);
    // }

    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    // let fifth = v[4];
    let fifth = v.pop().unwrap();

    // let third = v[2];
    let second = v.swap_remove(1);

    let third = std::mem::replace(&mut v[2], "substitute".to_string());

    let v = vec![
        "liberté".to_string(),
        "égalité".to_string(),
        "fraternité".to_string(),
    ];

    for mut s in v {
        s.push('!');
        println!("{}", s);
    }

    let l = Label { number: 3 };
    print(l);
    println!("Number of metka: {}", l.number);
    

}

#[derive(Clone)]
struct Label { number: i32 }
fn print(l: Label) {
    println!("STAMP: {}", l.number);
}

fn f(x: Vec<i32>) {}

fn g(x: Vec<i32>) {}

fn h(x: Vec<i32>) {}
