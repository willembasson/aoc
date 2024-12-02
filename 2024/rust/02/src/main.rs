const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}

fn get_numbers(line: &str) -> Vec<u8> {
    line.split_whitespace()
        .map(|num| num.parse::<u8>().unwrap())
        .collect()
}

fn check(values: &Vec<u8>) -> bool {
    let mut inc = true;
    let mut dec = true;
    for n in values.windows(2) {
        if n[0] > n[1] && (n[0] - n[1]) > 3 {
            return false;
        } else if n[1] > n[0] && (n[1] - n[0]) > 3 {
            return false;
        }
        inc = inc && n[0] < n[1];
        dec = dec && n[0] > n[1];
        if !(inc || dec) {
            return false;
        }
    }
    true
}

fn part1(input: &str) -> String {
    input
        .lines()
        .filter(|line| check(&get_numbers(line)))
        .count()
        .to_string()
}

fn part2(input: &str) -> String {
    input
        .lines()
        .filter(|line| {
            let values = get_numbers(line);
            check(&values)
                || (0..values.len()).any(|i| {
                    let mut values = values.clone();
                    values.remove(i);
                    check(&values)
                })
        })
        .count()
        .to_string()
}
