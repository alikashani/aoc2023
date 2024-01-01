use std::collections::HashMap;
use itertools::Itertools;

fn parse_line(line: &str) -> (String, (String, String)) {
    let value_and_left_right = line.split("=")
        .filter(|&e| e != "")
        .collect::<Vec<_>>();
    let key = value_and_left_right[0].replace(" ", "");
    let l_and_r = value_and_left_right[1].split(",")
        .filter(|&e| e != "")
        .map(|v| v.chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>())
        .collect_tuple::<(String, String)>()
        .unwrap();
    (key, l_and_r)
}

fn build_network(network_str: &str) -> HashMap<String, (String, String)> {
    let mut network = HashMap::new();
    for line in network_str.split("\n") {
        let (key, leftright) = parse_line(line);
        network.insert(key, leftright);
    }
    network
}

fn main() {
    let content = std::fs::read_to_string("input.txt")
        .expect("Failed to read file!");
    let content = content
        .split("\n\n")
        .collect::<Vec<_>>();
    
    let directions = content[0].split("")
        .filter(|&e| e != "")
        .collect::<Vec<_>>();

    let mut steps = 0usize;
    let mut found = false;
    let network = build_network(content[1]);

    let start = content[1].lines()
        .find(|&line| {
            let (key, _) = parse_line(line);
            key == "AAA"
        })
        .unwrap();

    let starting_points = content[1].lines()
        .map(parse_line)
        .filter(|(k, _)| k.ends_with("A"))
        .collect::<Vec<_>>();

    let (mut location, _) = parse_line(start);

    while !found {
        for &direction in directions.iter() {
            let mut zcount = 0;
            for (k, (l, r)) in starting_points.iter() {
                println!("k = {}", k);
                println!("Left = {}", l);
                println!("Righ = {}", r);
                let location = (if direction == "L" { l } else { r }).to_string();
                if location.ends_with("Z") {
                    zcount += 1;
                }
            }
            steps += 1;
            println!("***********");
            if zcount == starting_points.len() {
                found = true;
                break;
            }
        }
    }

    // while !found {
    //     for &direction in directions.iter() {
    //         if found {
    //             break;
    //         }
    //         let (l, r) = network.get(&location).unwrap();
    //         location = (if direction == "L" { l } else { r }).to_string();
    //         if location == "ZZZ" {
    //             found = true;
    //         }
    //         steps += 1;
    //     }
    // };

    println!("ANS: {}", steps);
}
