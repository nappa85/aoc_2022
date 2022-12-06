const INPUT: &[u8] = include_bytes!("../input");
const NUM_LINES: usize = get_num_lines();
const LINES: [(usize, usize); NUM_LINES] = get_lines();
const FIRST: u16 = first();
const SECOND: u16 = second();

const fn get_num_lines() -> usize {
    let mut i = 0;
    let mut o = 0;
    while i < INPUT.len() {
        if INPUT[i] == 10 {
            o += 1;
        }
        i += 1;
    }
    o
}

const fn get_lines() -> [(usize, usize); NUM_LINES] {
    let mut i = 0;
    let mut l = 0;
    let mut o = [(0, 0); NUM_LINES];
    while i < INPUT.len() {
        if INPUT[i] == 10 {
            o[l].1 = i;
            l += 1;
            if l < NUM_LINES {
                o[l].0 = i + 1;
            }
        }
        i += 1;
    }
    o
}

const fn first() -> u16 {
    let mut o = 0;
    let mut l = 0;
    while l < NUM_LINES {
        let middle = (LINES[l].0 + LINES[l].1) / 2;
        let mut i = LINES[l].0;
        'top: while i < middle {
            let mut j = middle;
            while j < LINES[l].1 {
                if INPUT[i] == INPUT[j] {
                    o += to_priority(INPUT[i]);
                    break 'top;
                }
                j += 1;
            }
            i += 1;
        }
        l += 1;
    }
    o
}

const fn second() -> u16 {
    let mut o = 0;
    let mut l = 0;
    while l < NUM_LINES {
        let mut i = LINES[l].0;
        'top: while i < LINES[l].1 {
            let mut j = LINES[l + 1].0;
            while j < LINES[l + 1].1 {
                if INPUT[i] == INPUT[j] {
                    let mut z = LINES[l + 2].0;
                    while z < LINES[l + 2].1 {
                        if INPUT[i] == INPUT[z] {
                            o += to_priority(INPUT[i]);
                            break 'top;
                        }
                        z += 1;
                    }
                }
                j += 1;
            }
            i += 1;
        }
        l += 3;
    }
    o
}

fn main() {
    println!("first {FIRST}");
    println!("second {SECOND}");
}

const fn to_priority(c: u8) -> u16 {
    match c {
        i if i >= 97 => i as u16 - 96, // a
        i if i >= 65 => i as u16 - 38, // A
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn to_priority() {
        assert_eq!(super::to_priority(b'a'), 1);
        assert_eq!(super::to_priority(b'A'), 27);
    }

    #[test]
    fn first() {
        let s = include_str!("../input");
        let sum = s
            .lines()
            .map(|r| r.split_at(r.len() / 2))
            .map(|(s1, s2)| s1.chars().find(|c| s2.contains(*c)).unwrap())
            .map(|c| super::to_priority(c as u8))
            .sum::<u16>();
        assert_eq!(sum, super::FIRST); // 8072
    }

    #[test]
    fn second() {
        let s = include_str!("../input");
        let lines = s.lines().collect::<Vec<_>>();
        let sum = lines
            .chunks(3)
            .flat_map(|s| {
                s[0].chars()
                    .find(|c| s[1].contains(*c) && s[2].contains(*c))
            })
            .map(|c| super::to_priority(c as u8))
            .sum::<u16>();
        assert_eq!(sum, super::SECOND); // 2567
    }
}
