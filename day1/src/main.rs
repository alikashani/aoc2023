const NUMS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine"
];

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    println!("Argument = {}", s);
    println!("Bytes = {:?}", bytes);
    println!("{}", u8::MIN);
    println!("{}", u8::MAX);
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("{i} {item}");
            return i;
        }
    }
    s.len()
}

#[inline(always)]
fn num(line: &[u8], i: usize) -> Option<usize> {
    // line[i].is_ascii_digit()
    //     .then_some((line[i] - b'0') as usize)
    //     .or(NUMS.iter()
    //         .enumerate()
    //         .find(|(_, name)| line[i..].starts_with(name))
    //         .map(|(n, _)| n + 1))

    if line[i].is_ascii_digit() {
        println!("{:?}", line[i]);
        println!("{:?}", line[i] - b'0');
        println!("---");
        Some((line[i] - b'0') as usize)
    } else {
        NUMS.iter()
            .enumerate()
            .find(|(_, name)| line[i..].starts_with(name))
            .map(|(n, _)| n + 1)
    }
}

fn main() {
    // secret = ownerproof-3846147-1702362853-efd416ee4216
    // value = first digit + last digit
    // 
    let result = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .map(|line| {
            (0..line.len()).find_map(|i| num(line, i)).unwrap() * 10
                + (0..line.len()).rev().find_map(|i| num(line, i)).unwrap()
        })
        .sum::<usize>();
    println!("Result = {:#?}", result);

    let sentence = String::from("Orange sauce is delicious!");
    println!("first_word Result = {:#?}", first_word(&sentence));
}
