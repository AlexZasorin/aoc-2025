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

fn main() {
    println!("Count: {}", part_one());
}
