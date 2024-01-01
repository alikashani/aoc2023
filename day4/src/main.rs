use std::collections::HashMap;

fn main() {
    let mut times_copied: HashMap<usize, usize> = HashMap::new();

    let content = std::fs::read_to_string("input.txt")
        .expect("Cannot read file.");

    let cards = content.lines()
        .map(|line| line.split(":")
            .last()
            .unwrap()
            .split("|")
            .map(|nums_in_str| nums_in_str.split(" ")
                .filter(|n| *n != "")
                .collect::<Vec<_>>())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for (i, card) in cards.iter().enumerate() {
        let card_id = i + 1;
        let winning = card.get(0).unwrap();

        let wins = card.get(1).unwrap()
            .iter()
            .filter(|n| winning.contains(n))
            .count();

        let copies = ((card_id+1)..=(card_id + wins))
            .collect::<Vec<usize>>();

        let own_times = match times_copied.get(&card_id) {
            Some(n) => n.clone(),
            None => 0
        };

        for copy in copies.iter() {
            match times_copied.get(&copy) {
                Some(prev) => {
                    times_copied.insert(*copy, prev + 1 + own_times);
                },
                None => {
                    times_copied.insert(*copy, 1);
                }
            }
        }

        if !times_copied.contains_key(&card_id) {
            times_copied.insert(card_id, 0);
        }
    }

    let mut result2 = 0;
    for (_, copy_count) in times_copied.iter() {
        result2 += copy_count + 1
    }

    // 5250608 = too low
    println!("ANS: {result2}");

    let input = include_bytes!("../input.txt");
    let col = input.iter().position(|&b| b == b':').unwrap();
    let sep = input.iter().position(|&b| b == b'|').unwrap();
    let mut factors = [1usize; 256];
    println!(
        "{}",
        input
            .split(|&b| b == b'\n')
            .enumerate()
            .map(|(i, game)| {
                let factor = factors[i];
                let win_seq = &game[col + 1..sep];
                let win_count = game[sep + 1..]
                    .chunks_exact(3)
                    .map(|n| &n[1..])
                    .filter(|n| win_seq.chunks_exact(3).map(|n| &n[1..]).any(|c| &c == n))
                    .count();
                (i..i + win_count).for_each(|i| factors[i + 1] += factor);
                factor * win_count + 1
            })
            .sum::<usize>()
    );
}
