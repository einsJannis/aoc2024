#![feature(iter_collect_into)]

use std::io::{stdin, BufRead, BufReader};

mod day1;
mod fetch;

const DAY: [fn(Box<str>) -> i64; 24] = [
    day1::compute,
    dummy,
    dummy,
    dummy,
    dummy,
    dummy,
    dummy,
    dummy,
    dummy,
    dummy,
    dummy,
    dummy,
    dummy,
    dummy,
    dummy,
    dummy,
    dummy,
    dummy,
    dummy,
    dummy,
    dummy,
    dummy,
    dummy,
    dummy,
];

fn dummy(_: Box<str>) -> i64 {
    return 0;
}

#[tokio::main]
async fn main() {
    let day: u8 = u8::from_str_radix(
        &std::env::args()
            .nth(1)
            .expect("Please provide a valid aoc date as the first argument!"),
        10,
    )
    .expect("Please provide a valid aoc date as the first argument!");
    let input = fetch::input(day).await;
    println!("{}", DAY[(day - 1) as usize](input))
}
