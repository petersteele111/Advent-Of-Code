use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

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

    let mut total_power = 0;
    for (game_id, attempts) in &games {
        let min_cubes = find_minimum_cubes(&attempts);
        let power = min_cubes.values().product::<i32>();
        total_power += power;
        println!("Game {}: Minimum cubes required - {:?}, Power - {}", game_id, min_cubes, power);
    }

    println!("Total power of all games: {}", total_power);

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

fn find_minimum_cubes(attempts: &Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut min_cubes = HashMap::new();

    for (color, number) in attempts {
        let entry = min_cubes.entry(color.clone()).or_insert(0);
        *entry = std::cmp::max(*entry, *number);
    }

    min_cubes
}
