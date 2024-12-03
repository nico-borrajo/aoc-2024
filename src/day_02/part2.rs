
pub fn part2(input: &str) -> isize {
    input
        .lines()
        .map(|l| l.split(' ').map(str::parse::<isize>).map(Result::unwrap))
        .fold(0, |acc, n| acc + is_safe(n) as isize)
}

fn is_safe(numbers: impl Iterator<Item = isize>) -> bool {
    let mut numbers = numbers.peekable();

    let mut prev = numbers.next().unwrap();
    let second = *numbers.peek().unwrap();

    let increasing = (second - prev) > 0;
    let mut used_safety = false;

    for number in numbers {
        if !level_safe(prev, number, increasing) {
            if used_safety { return false; }
            used_safety = true;
        } else {
            prev = number;
        }
    }

    return true;
}

fn level_safe(first: isize, second: isize, increasing: bool) -> bool {
    let dif = second - first;
    
    if dif.abs() < 1 || dif.abs() > 3 {
        return false;
    }
    
    return (dif > 0) == increasing;
}
