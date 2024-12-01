const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut v1: Vec<usize> = Vec::new();
    let mut v2: Vec<usize> = Vec::new();
    let lines = INPUT.lines();
    for line in lines {
        let mut it = line.split_whitespace();
        let first = it.next().unwrap().parse::<usize>().unwrap();
        let last = it.next().unwrap().parse::<usize>().unwrap();
        v1.push(first);
        v2.push(last);
    }

    let mut sum: usize = 0;
    for i in v1 {
        sum += i * (v2.iter().filter(|&n| *n == i).count());
    }

    println!("{}", sum);
}
