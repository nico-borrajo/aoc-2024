use std::collections::HashMap;

pub fn part2(input: &str) -> isize {
    let (v, h): (Vec<_>, HashMap<_, _>) = input
        .lines()
        .map(|s| s.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<isize>().unwrap(), b.parse::<isize>().unwrap()))
        .fold(
            (Vec::new(), HashMap::new()),
            |(mut v, mut h), (a, b)| {
                v.push(a);
                *h.entry(b).or_insert(0) += 1;
                (v, h)
            }
        );

    v.iter().map(|a| a*h.get(a).unwrap_or(&0)).sum()
}
