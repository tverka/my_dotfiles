use std::{
    fmt::{Debug, Display},
    ops::Sub, os::unix::ucred::impl_linux,
};

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify(item1: &impl Summary, item2: &impl Summary);
// pub fn notify<T1: Summary, T2: Summary>(item1: &T1, item2: &T2);

// pub fn notify(item: &(impl Display + Summary));
// pub fn notify<T: Summary + Display>(item: &T);

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32;

// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
// }

fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebook"),
        content: String::from(value)
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub contents: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
