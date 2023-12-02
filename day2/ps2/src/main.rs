use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Game {
    id: u32,
    counts: Vec<Count>,
}

#[derive(Debug)]
struct Count {
    blue_count: u32,
    red_count: u32,
    green_count: u32,
}

impl Game {
    fn new(id: u32, counts: Vec<Count>) -> Self {
        Game { id, counts }
    }

    fn max_counts(&self) -> (u32, u32, u32) {
       let mut max_blue = 0;
       let mut max_red = 0;
       let mut max_green = 0;

       for count in &self.counts {
           if count.blue_count > max_blue {
               max_blue = count.blue_count;
           }
           if count.red_count > max_red {
               max_red = count.red_count;
           }
           if count.green_count > max_green {
               max_green = count.green_count;
           }
       }

       (max_blue, max_red, max_green)
    }
}

fn main() -> io::Result<()> {
    let path = Path::new("./input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut games: Vec<Game> = Vec::new();

    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            let mut parts = line.split(": ");
            if let (Some(id), Some(count_str)) = (parts.next(), parts.next()) {
                let id_numeric = id.trim_start_matches("Game ").parse::<u32>().unwrap_or_default();
                let counts: Vec<Count> = count_str
                    .split("; ")
                    .map(|game_str| {
                        let mut counts = Count {
                            blue_count: 0,
                            red_count: 0,
                            green_count: 0,
                        };
                        for count in game_str.split(", ") {
                            let mut parts = count.split(' ');
                            if let (Some(number_str), Some(color)) = (parts.next(), parts.next()) {
                                if let Ok(number) = number_str.parse::<u32>() {
                                    match color {
                                        "blue" => counts.blue_count = number,
                                        "red" => counts.red_count = number,
                                        "green" => counts.green_count = number,
                                        _ => {}
                                    }
                                }
                            }
                        }
                        counts
                    })
                    .collect();
                games.push(Game::new(id_numeric, counts));
            }
        }
    }

    let mut sum = 0;

    for game in &games {
        let (max_blue, max_red, max_green) = game.max_counts();
        sum += max_red * max_blue * max_green;
    }
    println!("{}", sum);

    Ok(())
}

