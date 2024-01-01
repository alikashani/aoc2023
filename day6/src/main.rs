// Time:      7  15   30  (milliseconds (ms))
// Distance:  9  40  200  (millimeters  (mm))

// Boat:   starting speed (s) of 0 mm/ms; s = 0 mm/ms
// Button: 1 ms => s += 1 mm/ms

// Race 1 is 7 ms
//   0 ms =  0 mm
//   1 ms =  6 mm
//   2 ms = 10 mm
//   3 ms = 12 mm
//   4 ms = 12 mm
//   5 ms = 10 mm
//   6 ms =  6 mm
//   7 ms =  0 mm

// Determine # ways to beat record
// Then multiply # ways to get ANS

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn is_winning_hold(&self, hold: &usize) -> bool {
        (self.time - hold) * hold > self.distance
    }

    fn num_ways_to_win(&self) -> usize {
        (1..(self.time - 1)).filter(|n| self.is_winning_hold(n)).count()
    }
}

impl From<&Vec<usize>> for Race {
    fn from(value: &Vec<usize>) -> Self {
        Self {
            time: value[0],
            distance: value[1]
        }
    }
}

fn main() {
    let parsed = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line
            .split(" ")
            .skip(1)
            .filter(|e| *e != "")
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let ans = transpose(parsed)
        .iter()
        .map(Race::from)
        .map(|race| race.num_ways_to_win())
        .reduce(|acc, n| acc * n);
    
    println!("{:?}", ans);
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter()
            .map(|inner| inner[i].clone())
            .collect::<Vec<T>>())
        .collect()
}
