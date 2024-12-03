
pub fn part1(input: &str) -> isize {
    // your code goes here...
    input
        .lines()
        .map(|l| l.split(' ').map(str::parse::<isize>).map(Result::unwrap))
        .fold(0, |acc, n| acc + is_safe(n) as isize)
}

fn is_safe(mut numbers: impl Iterator<Item = isize>) -> bool {
    let first = numbers.next().unwrap();
    let mut prev = numbers.next().unwrap();

    
    let dif = prev - first;

    
    if dif.abs() < 1 || dif.abs() > 3 {
        return false;
    }
    
    let increasing = dif > 0;

    for number in numbers {
        let dif = number - prev;
        
        if dif.abs() < 1 || dif.abs() > 3 {
            return false;
        }
        
        if (dif > 0) != increasing { return  false; }

        prev = number;
    }

    return true;
}
