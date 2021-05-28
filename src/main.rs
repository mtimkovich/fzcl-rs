use chrono::{DateTime, Local, Timelike};
use std::collections::HashMap;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate maplit;

lazy_static! {
    static ref WORDS: HashMap<u32, &'static str> = {
        hashmap!{
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            10 => "ten",
            11 => "eleven",
            12 => "twelve",
            15 => "quarter",
            20 => "twenty",
            25 => "twenty-five",
            30 => "half",
        }
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
        format!("{} o'clock", words(hours))
    } else {
        format!("{} {} {}", words(rounded), glue, words(hours))
    }
}

fn main() {
    println!("{}", fuzzy(Local::now()));
}
