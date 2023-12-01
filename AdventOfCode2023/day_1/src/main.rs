
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./values.txt") {
        let total_sum = process_lines(lines);
        println!("Total Sum: {}", total_sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_lines(lines: io::Lines<io::BufReader<File>>) -> i32 {
    let mut total_sum = 0;

    for line in lines {
        if let Ok(x) = line {
            let first_digit = x.chars().find(|c| c.is_digit(10));
            let last_digit = x.chars().rev().find(|c| c.is_digit(10));

            if let (Some(first), Some(last)) = (first_digit, last_digit) {
                if let Ok(number) = format!("{}{}", first, last).parse::<i32>() {
                    total_sum += number;
                }
            }
        }
    }

    total_sum
}

