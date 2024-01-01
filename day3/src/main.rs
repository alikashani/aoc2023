fn is_part_number(grid: &Vec<Vec<char>>, j_range: (usize, usize), i: usize) -> bool {
    let (start, end) = j_range;
    let row = grid[i].len();
    let col = grid.len();

    let i_bounds = match i {
        0 => 0..=1,
        _ => match i == (col-1) {
            true => (i-1)..=i,
            false => (i-1)..=(i+1)
        }
    };
    let j_lower_bound = match start {
        0 => 0,
        _ => start - 1
    };
    let j_upper_bound = match end == (row-1) {
        true => end,
        false => end + 1
    };

    for ii in i_bounds {
        for jj in j_lower_bound..=j_upper_bound {
            let val = grid[ii][jj];
            if val != '.' && !(ii == i && (jj >= start && jj <= end)) {
                return true;
            }
        }
    }
    false
}

fn main() {
    let content = std::fs::read_to_string("input.txt")
        .expect("Cannot read file `input.txt`");

    let mut part_nums: Vec<i32> = Vec::new();

    let grid = content.lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for (i, line) in grid.iter().enumerate() {
        let mut part_maybe = String::from("");
        let mut started = false;
        let mut start = 0;
        for (j, ch) in line.iter().enumerate() {
            if ch.is_ascii_digit() {
                part_maybe += &ch.to_string();
                if !started {
                    started = true;
                    start = j
                }
            } else {
                if part_maybe != "" {
                    if is_part_number(&grid, (start, j-1), i) {
                        part_nums.push(part_maybe.parse::<i32>().unwrap());
                    }
                    part_maybe = String::from("");
                    started = false;
                }
            }
        }
    }

    println!("ANS: {:#?}", part_nums.iter().sum::<i32>());
}
