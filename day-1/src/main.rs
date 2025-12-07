use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

fn part_one() -> u32 {
    let mut dial: u32 = 50;
    let mut count: u32 = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.map_while(Result::ok) {
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
    }

    count
}

fn part_two() -> u32 {
    let mut dial = 50;
    let mut count = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.map_while(Result::ok) {
            let dir = &line.chars().next().unwrap();
            let mut num = line[1..].parse::<i32>().unwrap();

            if *dir == 'L' {
                let mut start_zero = dial == 0;
                while num > dial {
                    num -= dial + 1;
                    dial = 99;
                    count += if !start_zero { 1 } else { 0 };
                    start_zero = false;
                }

                dial -= num;
                count += if dial == 0 && !start_zero { 1 } else { 0 };
            } else {
                while num + dial > 99 {
                    num -= 100 - dial;
                    dial = 0;
                    count += 1;
                }

                dial += num;
            }
        }
    }

    count
}

fn main() {
    println!("Part 1 Count: {}", part_one());
    println!("Part 2 Count: {}", part_two());
}
