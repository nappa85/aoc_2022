const INPUT: &[u8] = include_bytes!("../input");
const SPLIT_LINE: usize = get_split_line();
const NUM_LINES: usize = get_num_lines();
const CRATES: [[Option<char>; NUM_LINES * 6]; 9] = get_crates();
const NUM_MOVES: usize = get_num_moves();
const MOVES: [(usize, usize, usize); NUM_MOVES] = get_moves();
const FIRST: [char; 9] = first();
const SECOND: [char; 9] = second();

const fn get_split_line() -> usize {
    let mut i = 0;
    while i < INPUT.len() - 1 {
        if INPUT[i] == 10 && INPUT[i + 1] == 10 {
            return i;
        }
        i += 1;
    }
    unreachable!();
}

const fn get_num_lines() -> usize {
    let mut i = 0;
    let mut o = 0;
    while i < SPLIT_LINE {
        if INPUT[i] == 10 {
            o += 1;
        }
        i += 1;
    }
    o
}

// we elarge the buffer to accept future moves
const fn get_crates() -> [[Option<char>; NUM_LINES * 6]; 9] {
    let mut o = [[None; NUM_LINES * 6]; 9];
    //start from top
    let mut slot = NUM_LINES - 1;
    let mut i = 0;
    loop {
        let mut next_line = i;
        while INPUT[next_line] != 10 {
            next_line += 1;
        }
        i += 1;
        let mut c = 0;
        while i < next_line {
            if INPUT[i] != 32 {
                o[c][slot] = Some(INPUT[i] as char);
            }
            i += 4;
            c += 1;
        }
        i = next_line + 1;
        if slot == 0 {
            break;
        } else {
            slot -= 1;
        }
    }
    o
}

const fn get_num_moves() -> usize {
    let mut i = SPLIT_LINE + 2;
    let mut o = 0;
    while i < INPUT.len() {
        if INPUT[i] == 10 {
            o += 1;
        }
        i += 1;
    }
    o
}

const fn get_moves() -> [(usize, usize, usize); NUM_MOVES] {
    let mut i = SPLIT_LINE + 2;
    let mut l = 0;
    let mut o = [(0, 0, 0); NUM_MOVES];
    while l < NUM_MOVES {
        o[l] = (
            get_num(i + 5, i + 6),
            get_num(i + 12, i + 13),
            get_num(i + 17, i + 18),
        );
        while INPUT[i] != 10 {
            i += 1;
        }
        i += 1;
        l += 1;
    }
    o
}

const fn get_num(a: usize, b: usize) -> usize {
    let mut mul = 1;
    let mut o = if INPUT[b] >= b'0' {
        mul = 10;
        INPUT[b] - b'0'
    } else {
        0
    };
    if INPUT[a] >= b'0' {
        o += (INPUT[a] - b'0') * mul;
    };
    o as usize
}

const fn first() -> [char; 9] {
    let mut crates = CRATES;
    let mut m = 0;
    while m < NUM_MOVES {
        let (mut take, from, to) = MOVES[m];
        while take > 0 {
            let a = get_last_full(&crates[from - 1]);
            let b = get_first_empty(&crates[to - 1]);
            crates[to - 1][b] = crates[from - 1][a];
            crates[from - 1][a] = None;
            take -= 1;
        }
        m += 1;
    }
    let mut out = [' '; 9];
    let mut i = 0;
    while i < 9 {
        if let Some(c) = crates[i][get_last_full(&crates[i])] {
            out[i] = c;
        }
        i += 1;
    }
    out
}

const fn second() -> [char; 9] {
    let mut crates = CRATES;
    let mut m = 0;
    while m < NUM_MOVES {
        let (mut take, from, to) = MOVES[m];
        let orig_take = take;
        let a = get_last_full(&crates[from - 1]);
        let b = get_first_empty(&crates[to - 1]);
        while take > 0 {
            crates[to - 1][b + (take - 1)] = crates[from - 1][a - (orig_take - take)];
            crates[from - 1][a - (orig_take - take)] = None;
            take -= 1;
        }
        m += 1;
    }
    let mut out = [' '; 9];
    let mut i = 0;
    while i < 9 {
        if let Some(c) = crates[i][get_last_full(&crates[i])] {
            out[i] = c;
        }
        i += 1;
    }
    out
}

const fn get_last_full(c: &[Option<char>]) -> usize {
    let mut i = 0;
    while i < c.len() {
        if c[i].is_none() {
            return i - 1;
        }
        i += 1;
    }
    unreachable!()
}

const fn get_first_empty(c: &[Option<char>]) -> usize {
    let mut i = 0;
    while i < c.len() {
        if c[i].is_none() {
            return i;
        }
        i += 1;
    }
    unreachable!()
}

fn main() {
    println!("contained {FIRST:?}");
    println!("overlapping {SECOND:?}");
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    fn get_lines() -> impl Iterator<Item = &'static str> {
        include_str!("../input").lines()
    }

    fn get_crates() -> Vec<VecDeque<&'static str>> {
        get_lines()
            .take_while(|s| !s.is_empty())
            .fold(vec![VecDeque::new(); 9], |mut piles, s| {
                if s.contains('[') {
                    for i in 0..9 {
                        let c = s.get(((i * 4) + 1)..((i * 4) + 2));
                        if let Some(c) = c {
                            if c != " " {
                                piles[i].push_front(c);
                            }
                        }
                    }
                }
                piles
            })
    }

    fn get_moves() -> Vec<(usize, usize, usize)> {
        get_lines()
            .skip_while(|s| !s.is_empty())
            .skip(1)
            .map(|s| {
                let mut parts = s.split(' ');
                let take = parts.clone().nth(1).unwrap().parse::<usize>().unwrap();
                let from = parts.clone().nth(3).unwrap().parse::<usize>().unwrap();
                let to = parts.nth(5).unwrap().parse::<usize>().unwrap();
                (take, from, to)
            })
            .collect()
    }

    #[test]
    fn first() {
        let mut crates = get_crates();
        let moves = get_moves();

        moves.iter().for_each(|(take, from, to)| {
            for _ in 0..*take {
                let temp = crates[from - 1].pop_back().unwrap();
                crates[to - 1].push_back(temp);
            }
        });
        let res = crates
            .into_iter()
            .flat_map(|c| c.back().unwrap().chars())
            .collect::<Vec<_>>();
        assert_eq!(res, super::FIRST); //FCVRLMVQP
    }

    #[test]
    fn second() {
        let mut crates = get_crates();
        let moves = get_moves();
        moves.iter().for_each(|(take, from, to)| {
            let mut temp = Vec::with_capacity(*take);
            for _ in 0..*take {
                let t = crates[from - 1].pop_back().unwrap();
                temp.push(t);
            }
            for t in temp.into_iter().rev() {
                crates[to - 1].push_back(t);
            }
        });
        let res = crates
            .into_iter()
            .flat_map(|c| c.back().unwrap().chars())
            .collect::<Vec<_>>();
        assert_eq!(res, super::SECOND); //RWLWGJGFD
    }
}
