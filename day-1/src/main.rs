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
            println!("Dial: {}", dial);
            println!("Left {}", num);
            while num > dial {
                // If char is L, check if number is > dial
                //  If number is > dial, loop, setting dial to 100 and subtracting dial from number, until
                //  number is < dial
                num -= (dial + 1);
                dial = 99;

                println!("Dial: {}, Remaining: {}", dial, num);
            }

            dial -= num;
            println!("Dial: {}", dial);
        } else {
            println!("Dial: {}", dial);
            println!("Right {}", num);
            dial = (dial + num) % 100;
            println!("Dial: {}", dial);
        }

        count += if dial == 0 { 1 } else { 0 };
    }

    println!("Count: {}", count);
}
