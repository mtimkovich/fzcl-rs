use chrono::{DateTime, Local, Timelike};
use std::collections::HashMap;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref WORDS: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(1, "one");
        m.insert(2, "two");
        m.insert(3, "three");
        m.insert(4, "four");
        m.insert(5, "five");
        m.insert(6, "six");
        m.insert(7, "seven");
        m.insert(8, "eight");
        m.insert(9, "nine");
        m.insert(10, "ten");
        m.insert(11, "eleven");
        m.insert(12, "twelve");
        m.insert(15, "quarter");
        m.insert(20, "twenty");
        m.insert(30, "half");
        m
    };
}

fn words(key: u32) -> &'static str {
    WORDS.get(&key).unwrap()
}

fn round(minutes: u32) -> (u32, bool) {
    let mut rounded = (minutes + 2) % 60 / 5 * 5;
    let mut up = false;

    if rounded > 35 {
        rounded = 60 - rounded;
        up = true;
    } else if minutes > 30 && rounded == 0 {
        up = true;
    }

    if rounded >= 25 && rounded <= 35 {
        rounded = 30;
    }

    (rounded, up)
}

fn fuzzy(now: DateTime<Local>) -> String {
    let (_, mut hours) = now.hour12();
    let minutes = now.minute();
    let mut glue = "past";
    let (rounded, up) = round(minutes);

    if up {
        hours += 1;
        glue = "til";
    }

    if rounded == 0 {
        return format!("{} o'clock", words(hours));
    } else {
        return format!("{} {} {}", words(rounded), glue, words(hours));
    }
}

fn main() {
    println!("{}", fuzzy(Local::now()));
}
