fn is_island(
    num_string: String,
    line_number: usize,
    range: (usize, usize),
    grid: &Vec<Vec<char>>
) -> bool {
    // determine if all adjacent symbols are "." characters
    let mut threshold = ((num_string.len() + 2) * 3) - num_string.len();
    let mut water_found = 0;
    let (lo, hi) = range;

    // dfs or bfs?
    for val in &grid[line_number + 1] {
        if val.to_string().eq(".") {
            water_found += 1;
        }
        println!("{val}");
    }
    // line_number = 0?         check line 0 & 1
    // line_number = 1?         check line 0, 1, 2
    // line_number = last line? check lines -1, -2

    water_found == threshold
}

fn main2() {
    let content = std::fs::read_to_string("input.txt")
        .expect("Cannot read file `input.txt`");

    let grid = content.lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let enumerated_lines = content
        .lines()
        .enumerate()
        .collect::<Vec<_>>();

    let result = enumerated_lines.iter()
        .map(|(i, line)| {
            // parse for num
            println!("{i} --> {line}");

            let mut started = false;
            let mut i_start = 0;
            // let mut i_end = 0;
            let mut num_string = String::from("");

            for (j, ch) in line.char_indices() {
                if ch.is_ascii_digit() {
                    if !started {
                        started = true;
                        i_start = j;
                    }
                    num_string += &ch.to_string()
                } else {
                    let is_end = j == line.len() - 1;
                    if ch.to_string().eq(".") | is_end {
                        match num_string.parse::<i32>() {
                            Ok(n) => {
                                println!("{} = ({}, {})", n, i_start, (j-1));
                                let is_part_number = is_island(
                                    num_string,
                                    *i,
                                    (i_start, j - 1),
                                    &grid
                                );
                                println!("is part number = {is_part_number}");
                            },
                            Err(_) => {
                                println!("Err :: {}, {}", num_string, i_start);
                            }
                        }
                        num_string = String::from("");
                        started = false;
                        i_start = 0;
                    }
                }
            }

            println!("{num_string:?}");
            println!("--------------------------------");
            // determine if num is adjacent
            // sum adjacent nums
            line
        })
        .collect::<Vec<_>>();
    println!("{:#?}", result);
}
