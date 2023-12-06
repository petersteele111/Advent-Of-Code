use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashSet, HashMap};

fn process_cards(cards: Vec<String>) -> i32 {
    let mut card_copies = HashMap::new();
    let mut total_cards = 0;

    for index in 0..cards.len() {
        card_copies.insert(index, 1);
    }

    let mut index = 0;
    while index < cards.len() {
        let copies = *card_copies.get(&index).unwrap_or(&0);
        let card = &cards[index];

        let parts: Vec<&str> = card.split('|').collect();
        let winning_numbers: HashSet<i32> = parts[0].split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();
        let your_numbers: HashSet<i32> = parts[1].split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let matches = winning_numbers.intersection(&your_numbers).count();

        for i in 1..=matches {
            if index + i < cards.len() {
                *card_copies.entry(index + i).or_insert(0) += copies;
            }
        }

        total_cards += copies;
        index += 1;
    }

    total_cards
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

    let total_scratchcards = process_cards(cards);
    println!("Total scratchcards: {}", total_scratchcards);

    Ok(())
}
