const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut v1: Vec<u64> = Vec::new();
    let mut v2: Vec<u64> = Vec::new();
    let mut lines = INPUT.lines();
    while let Some(line) = lines.next() {
        let mut it = line.split_whitespace();
        let first = it.next().unwrap().parse::<u64>();
        let last = it.next().unwrap().parse::<u64>();
        v1.push(first.clone().unwrap());
        v2.push(last.clone().unwrap());
    }

    let mut sum: u64 = 0;
    for i in 0..v1.len() {
        sum += v1[i] * (v2.iter().filter(|&n| *n == v1[i]).count()) as u64;
    }
    println!("{}", sum);
}
