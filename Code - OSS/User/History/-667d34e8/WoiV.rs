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

    struct Point { x: i32, y: i32 };
    let point = Point { x: 100, y: 200 };
    let r: &Point = &point;
    let rr = &r;
    let rrr = &rr;
    
}

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