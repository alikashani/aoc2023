use std::ops::Index;

const RED_TARGET: u32 = 12;
const GREEN_TARGET: u32 = 13;
const BLUE_TARGET: u32 = 14;

const RED: &str = "red";
const GREEN: &str = "green";
const BLUE: &str = "blue";

#[derive(Debug)]
struct Reveal {
    red: u32,
    green: u32,
    blue: u32
}

#[derive(Debug)]
struct Game {
    id: u32,
    reveals: Vec<Reveal>
}

// Game 1
// 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn get_game_id(game_string: &str) -> u32 {
    game_string
        .split(":")
        .collect::<Vec<&str>>()
        .get(0)
        .unwrap()
        .split(" ")
        .filter(|s| s.len() > 0)
        .collect::<Vec<_>>()
        .get(1)
        .unwrap()
        .parse::<u32>()
        .unwrap()
}

fn get_reveals(game_string: &str) -> Vec<Reveal> {
    game_string
        .split(":")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .split(";")
        .map(Reveal::from)
        .collect::<Vec<_>>()
}

fn resolve_rev(color: &str, reveal_string_part: &str) -> u32 {
    let count_and_color = reveal_string_part
        .split(" ")
        .filter(|s| s.len() > 0)
        .collect::<Vec<_>>();
    count_and_color.get(1)
        .unwrap()
        .contains(color)
        .then_some(count_and_color.get(0)
            .unwrap()
            .parse::<u32>()
            .unwrap())
        .unwrap_or(0)
}

impl From<&str> for Game {
    fn from(game_string: &str) -> Self {
        Self {
            id: get_game_id(game_string),
            reveals: get_reveals(game_string)
        }
    }
}

impl From<&str> for Reveal {
    fn from(reveal_string: &str) -> Self {
        let mut red = 0u32;
        let mut green = 0u32;
        let mut blue = 0u32;
        for rev in reveal_string.split(",") {
            red = red | resolve_rev(RED, rev);
            green = green | resolve_rev(GREEN, rev);
            blue = blue | resolve_rev(BLUE, rev);
        }
        Self { red, green, blue }
    }
}

impl Index<&str> for Reveal {
    type Output = u32;

    fn index(&self, color: &str) -> &Self::Output {
        match color {
            RED => &self.red,
            GREEN => &self.green,
            BLUE => &self.blue,
            _ => panic!("Invalid color provided!")
        }
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        self.reveals.iter()
            .filter(|rev| {
                rev.red.le(&RED_TARGET) &
                rev.green.le(&GREEN_TARGET) &
                rev.blue.le(&BLUE_TARGET)
            })
            .collect::<Vec<_>>()
            .len()
            .eq(&self.reveals.len())
    }

    fn max_color(&self, color: &str) -> u32 {
        self.reveals.iter()
            .map(|rev| rev[color])
            .max()
            .unwrap()
    }
}

fn main() {
    let games: Vec<Game> = std::fs::read_to_string("input.txt")
        .expect("Error reading file!")
        .split("\n")
        .map(|line| Game::from(line))
        .collect();

    let result1 = games.iter()
        .filter_map(|g| g.is_possible().then_some(g.id))
        .sum::<u32>();
    println!("Part 1 = {}", result1);

    let result2 = games.iter()
        .map(|g| g.max_color(RED) * g.max_color(GREEN) * g.max_color(BLUE))
        .sum::<u32>();
    println!("Part 2 = {}", result2);
}
