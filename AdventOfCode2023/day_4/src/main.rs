use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn calculate_points(cards: Vec<String>) -> i32 {
    let mut total_points = 0;

    for card in cards {
        let parts: Vec<&str> = card.split('|').collect();
        let winning_numbers: HashSet<i32> = parts[0].split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();
        let your_numbers: HashSet<i32> = parts[1].split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let matches = winning_numbers.intersection(&your_numbers).count();

        if matches > 0 {
            total_points += 1 << (matches - 1);
        }
    }

    total_points
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> io::Result<()> {
    let mut cards = Vec::new();

    if let Ok(lines) = read_lines("values.txt") {
        for line in lines {
            if let Ok(card) = line {
                cards.push(card);
            }
        }
    }

    let total_points = calculate_points(cards);
    println!("Total points: {}", total_points);

    Ok(())
}
