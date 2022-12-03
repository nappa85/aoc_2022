fn main() {
    let s = include_str!("../input");
    let sum = s
        .lines()
        .map(|r| r.split_at(r.len() / 2))
        .map(|(s1, s2)| s1.chars().find(|c| s2.contains(*c)).unwrap())
        .map(to_priority)
        .sum::<u32>();
    println!("{sum}"); // 4016
    let lines = s.lines().collect::<Vec<_>>();
    let sum = lines
        .chunks(3)
        .flat_map(|s| {
            s[0].chars()
                .find(|c| s[1].contains(*c) && s[2].contains(*c))
        })
        .map(to_priority)
        .sum::<u32>();
    println!("{sum}"); // 4016
}

fn to_priority(c: char) -> u32 {
    match c as u32 {
        i if i >= 97 => i - 96, // a
        i if i >= 65 => i - 38, // A
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn to_priority() {
        assert_eq!(super::to_priority('a'), 1);
        assert_eq!(super::to_priority('A'), 27);
    }
}
