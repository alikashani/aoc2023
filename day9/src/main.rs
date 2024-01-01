fn populate_history(curr: &Vec<i32>, hist: &mut Vec<Vec<i32>>) {
    if curr.len() == 0 {
        return;
    }

    let mut i = 0;
    let mut j = 1;
    let mut acc = Vec::with_capacity(curr.len() - 1);

    while j < curr.len() {
        acc.push(curr[j] - curr[i]);
        i += 1;
        j += 1;
    }

    hist.push(acc.clone());
    populate_history(&acc, hist);
}

fn main() {
    let sum = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let sequence = line.split(" ")
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            let mut history = Vec::new();
            populate_history(&sequence, &mut history);
            history.reverse();
            history.push(sequence);

            let mut i = 0;
            let mut j = 1;
            while j < history.len() {
                let a = history[i].clone();
                let b = &mut history[j];
                b.splice(..0, [b.first().unwrap_or(&0) - a.first().unwrap_or(&0)]);
                i += 1;
                j += 1;
            }
            history.last().unwrap().first().unwrap_or(&0).clone()
        })
        .sum::<i32>();

    println!("ANS: {sum}");
}
