use std::iter::zip;

pub fn part1(input: &str) -> isize {
    let (mut a, mut b): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|s| s.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<isize>().unwrap(), b.parse::<isize>().unwrap()))
        .unzip();

    a.sort();
    b.sort();

    zip(a, b).map(|(a, b)| (a - b).abs()).sum()
}
