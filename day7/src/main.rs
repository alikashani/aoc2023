use std::collections::HashMap;

const CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8',
    '9', 'T', 'J', 'Q', 'K', 'A'
];

#[derive(Debug)]
struct Hand {
    counts: HashMap<char, usize>,
    ranked_cards: usize,
    bid: usize,
    score: usize
}

impl Hand {
    fn is_five_oak(&self) -> bool {
        self.counts.values().find(|&v| *v == 5).is_some()
    }

    fn is_four_oak(&self) -> bool {
        self.counts.values().find(|&v| *v == 4).is_some()
    }

    fn is_full_house(&self) -> bool {
        self.is_three_oak() && self.is_pair()
    }

    fn is_three_oak(&self) -> bool {
        self.counts.values().find(|&v| *v == 3).is_some()
    }

    fn is_two_pair(&self) -> bool {
        self.counts.values().filter(|&v| *v == 2).count() == 2
    }

    fn is_pair(&self) -> bool {
        self.counts.values().find(|&v| *v == 2).is_some()
    }

    fn calc_score(&self) -> usize {
        if self.is_five_oak() { 7 }
        else if self.is_four_oak() { 6 }
        else if self.is_full_house() { 5 }
        else if self.is_three_oak() { 4 }
        else if self.is_two_pair() { 3 }
        else if self.is_pair() { 2 }
        else { 1 }
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let content = value.split(" ").collect::<Vec<_>>();

        let mut counts = HashMap::new();
        for card in content[0].chars() {
            match counts.get(&card) {
                Some(c) => counts.insert(card, c + 1),
                None => counts.insert(card, 1)
            };
        }

        let ranked_cards = content[0].chars()
            .map(|c| CARDS.iter()
                .position(|&e| e == c)
                .unwrap()
                .to_string())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let mut hand = Self {
            // raw: String::from(content[0]),
            counts,
            score: 0,
            ranked_cards,
            bid: content[1].parse().unwrap()
        };

        hand.score = hand.calc_score();
        hand
    }
}

// 248230746 = too low
// 248422077 = correct
// 250766353 = too high
fn main() {
    let mut hands = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(Hand::from)
        .collect::<Vec<_>>();

    // 1. Sort hands by strength + high card
    hands.sort_unstable_by_key(|h| (h.score, h.ranked_cards));

    // 2. Iterate over them and multiply by rank
    let result = hands.iter().enumerate()
        .map(|(i, h)| (i+1) * h.bid)
        .sum::<usize>();
    
    println!("{:#?}", hands);
    println!("ANS: {}", result);
}
