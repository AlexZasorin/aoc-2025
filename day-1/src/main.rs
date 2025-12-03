use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut dial: u32 = 50;
    let mut count: u32 = 0;

    let file = File::open("./input.txt").unwrap();

    for line in io::BufReader::new(file).lines().map_while(Result::ok) {
        let dir = &line.chars().next().unwrap();
        let mut num = line[1..].parse::<u32>().unwrap();

        if *dir == 'L' {
            while num > dial {
                num -= dial + 1;
                dial = 99;
            }

            dial -= num;
        } else {
            dial = (dial + num) % 100;
        }

        count += if dial == 0 { 1 } else { 0 };
    }

    println!("Count: {}", count);
}
