const INPUT: &[u8] = include_bytes!("../input");
const NUM_OP: usize = get_num_op();
const OPS: [i8; NUM_OP] = get_ops();
const FIRST: i32 = first();
const SECOND: [char; NUM_OP + NUM_OP / 40] = second();

const fn get_num_op() -> usize {
    let mut i = 0;
    let mut o = 1;
    while i < INPUT.len() {
        if INPUT[i] == b'n' {
            o += 1;
        } else if INPUT[i] == b'a' {
            o += 2;
        }
        while INPUT[i] != b'\n' {
            i += 1;
        }
        i += 1;
    }
    o
}

const fn get_ops() -> [i8; NUM_OP] {
    let mut i = 0;
    let mut o = [1; NUM_OP];
    let mut o_i = 1;
    while i < INPUT.len() {
        if INPUT[i] == b'n' {
            o[o_i] = o[o_i - 1];
            o_i += 1;
        } else if INPUT[i] == b'a' {
            o[o_i] = o[o_i - 1];
            o_i += 1;
            let temp = [INPUT[i + 5], INPUT[i + 6], INPUT[i + 7]];
            o[o_i] = o[o_i - 1] + get_i8(temp);
            o_i += 1;
        } else {
            unreachable!()
        }
        while INPUT[i] != b'\n' {
            i += 1;
        }
        i += 1;
    }
    o
}

const fn get_i8(input: [u8; 3]) -> i8 {
    let mut pow = 0;
    let mut i = 2;
    let mut o = 0;
    loop {
        match input[i] {
            n @ b'0'..=b'9' => {
                o += (n - b'0') as i8 * 10_i8.pow(pow);
                pow += 1;
            }
            b'-' => o *= -1,
            _ => {}
        }
        if i == 0 {
            break;
        } else {
            i -= 1;
        }
    }
    o
}

const fn first() -> i32 {
    let ids = [20, 60, 100, 140, 180, 220];
    let mut i = 0;
    let mut o = 0;
    while i < ids.len() {
        o += ids[i] * OPS[ids[i] as usize - 1] as i32;
        i += 1;
    }
    o
}

const fn second() -> [char; NUM_OP + NUM_OP / 40] {
    let mut o = ['.'; NUM_OP + NUM_OP / 40];
    let mut o_i = 0;
    let mut i = 0;
    while i < OPS.len() {
        let sub_i = (i % 40) as i8;
        if i > 0 && sub_i == 0 {
            o[o_i] = '\n';
            o_i += 1;
        }
        if OPS[i] >= sub_i - 1 && OPS[i] <= sub_i + 1 {
            o[o_i] = '#';
        }
        o_i += 1;
        i += 1;
    }
    o
}

fn main() {
    println!("{FIRST}");
    println!("{}", SECOND.into_iter().collect::<String>());
}

#[cfg(test)]
mod tests {
    enum Op {
        Noop,
        Addx(i8),
    }

    fn get_ops() -> Vec<i8> {
        let input = include_str!("../input");
        input
            .lines()
            .map(|s| match s {
                "noop" => Op::Noop,
                _ => Op::Addx(s[5..].parse().unwrap()),
            })
            .fold(vec![1], |mut a, op| {
                match op {
                    Op::Noop => {
                        let temp = *a.last().unwrap();
                        a.push(temp);
                    }
                    Op::Addx(n) => {
                        let temp = *a.last().unwrap();
                        a.push(temp);
                        a.push(temp + n)
                    }
                }
                a
            })
    }

    #[test]
    fn ops() {
        let v = get_ops();
        assert_eq!(v, super::OPS);
    }

    #[test]
    fn fist() {
        let v = get_ops();
        let sum: i32 = [20, 60, 100, 140, 180, 220]
            .into_iter()
            .map(|i| i as i32 * v[i as usize - 1] as i32)
            .sum();
        assert_eq!(sum, super::FIRST); // 14160
    }

    #[test]
    fn second() {
        let v = get_ops();
        let out = v
            .chunks(40)
            .flat_map(|chunk| {
                (0_i8..(chunk.len() as i8))
                    .map(|i| {
                        if ((i - 1)..=(i + 1)).contains(&(chunk[i as usize])) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .chain(if chunk.len() == 40 { Some('\n') } else { None })
            })
            .collect::<Vec<_>>();
        assert_eq!(out, super::SECOND);
        /*
        ###....##.####.###..###..####.####..##..
        #..#....#.#....#..#.#..#.#....#....#..#.
        #..#....#.###..#..#.#..#.###..###..#....
        ###.....#.#....###..###..#....#....#....
        #.#..#..#.#....#.#..#....#....#....#..#.
        #..#..##..####.#..#.#....####.#.....##..
        .
        */
    }
}
