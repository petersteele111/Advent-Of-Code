use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("values.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut games = vec![];
    
    for line in reader.lines() {
        let line = line?;
        if let Some(game_data) = parse_game_data(&line) {
            games.push(game_data);
        }
    }

    let result = calculate_possible_games_sum(&games);
    println!("Sum of possible game IDs: {}", result);

    Ok(())
}

fn parse_game_data(line: &str) -> Option<(i32, Vec<(String, i32)>)> {
    let parts: Vec<&str> = line.split(": ").collect();
    if parts.len() != 2 {
        return None;
    }

    let game_id = parts[0].replace("Game ", "").parse::<i32>().ok()?;
    let attempts = parts[1].split(';')
        .map(|s| {
            let counts = s.trim().split(',')
                .filter_map(|c| {
                    let parts: Vec<&str> = c.trim().split_whitespace().collect();
                    if parts.len() == 2 {
                        Some((parts[1].to_string(), parts[0].parse::<i32>().ok()?))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(String, i32)>>();
            counts
        })
        .collect::<Vec<Vec<(String, i32)>>>();

    Some((game_id, attempts.concat()))
}

fn calculate_possible_games_sum(games: &Vec<(i32, Vec<(String, i32)>)>) -> i32 {
    let max_cubes = [("red".to_string(), 12), ("green".to_string(), 13), ("blue".to_string(), 14)]
        .iter().cloned().collect::<std::collections::HashMap<_, _>>();

    games.iter()
        .filter(|&(_, attempts)| {
            let mut counts = std::collections::HashMap::new();
            for (color, number) in attempts {
                let count = counts.entry(color.clone()).or_insert(0);
                *count = std::cmp::max(*count, *number);
            }

            counts.iter().all(|(color, &count)| max_cubes.get(color).map_or(false, |&max| count <= max))
        })
        .map(|(id, _)| id)
        .sum()
}

