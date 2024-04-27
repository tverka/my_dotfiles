use std::{collections::HashMap, vec};

type Table = HashMap<String, Vec<String>>;

fn main() {
    let mut table = Table::new();
    table.insert(
        "Джезуальдо".to_string(),
        vec!["мадригалы".to_string(), "Tenebrae Responsoria".to_string()],
    );
    table.insert(
        "Караваджо".to_string(),
        vec![
            "Музыканты".to_string(),
            "Призвание апостола Матфея".to_string(),
        ],
    );
    table.insert(
        "Челлини".to_string(),
        vec!["Персей с головой Медузы".to_string(), "Сальера".to_string()],
    );
    show(&table);
    assert_eq!(table["Джезуальдо"][0], "мадригалы");
    assert_eq!(table["Джезуальдо"][0], "мадригалы");
    println!("----------SORTED----------");
    show(&table);

    println!("--------SECTION_1----------");
    let x = 10;
    let y = 20;
    let mut r = &x;

    struct Point {
        x: i32,
        y: i32,
    };
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr = &r;
    let rrr = &rr;
    assert_eq!(rrr.y, 729);

    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;
    assert!(rrx <= rry);
    assert!(rrx == rry);

    assert!(!std::ptr::eq(rrx, rry));

    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);

    println!("--------SECTION_2----------");

    // let r;
    {
        // let x = 1;
        // r = &x;
    }
    // assert_eq!(*r, 1);

    let x = 1;
    {
        let r = &x;
        assert_eq!(*r, 1);
    }

    let x = 10;
    g(&x);

    // let s;
    // {
    //     let parabola = [9, 4, 1, 0, 1, 4, 9];
    //     s = smallest(&parabola);
    // }
    // assert_eq!(*s, 0);

    let s;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        s = smallest(&parabola);
        assert_eq!(*s, 0);
    }

    let mut x = 10;
    let r1 = &x;
    let r2 = &x;
    x += 10;
    let m = &mut x;
    println!("x={x}");
    
    let mut y = 20;
    let m1 = &mut y;
    let m2 = &mut y;
    let z = y;

    let mut w = (107, 109);
    let r = &w;
    let r0 = &r.0;
    // let m1 = &mut r.1; error
    
    let mut v = (136, 139);
    let m = &mut v;
    let m0 = &mut m.0;
    *m0 = 137;
    let r1 = &m.1;
    println!("{}", v.1);

    let x;
    x = 10;
    // x = 25;
    


    
}

fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s { s = r; }
    }
    s
}

fn g<'a>(p: &'a i32) { }

static mut STASH: &i32 = &128;
fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

static WORTH_POINTING_AT: i32 = 1000;


fn show(table: &Table) {
    for (artist, works) in table {
        println!("Работы {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

fn factorial(n: usize) -> usize {
    (1..n + 1).fold(1, |a, b| a * b)
}
