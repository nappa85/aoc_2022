#![feature(const_eval_limit)]
#![const_eval_limit = "70000000"]

const INPUT: &[u8] = include_bytes!("../input");
const NUM_MONKE: usize = get_num_monke();
const ITEMS: usize = 100;
const MONKE: [Monke; NUM_MONKE] = get_monke();
const FIRST: u64 = first();
const SECOND: u64 = second();

#[derive(Copy, Clone, Debug)]
struct Monke {
    items: [Option<u64>; ITEMS],
    operation: Operation,
    test: u64,
    positive: usize,
    negative: usize,
    inspected: u64,
}

impl Monke {
    const fn new() -> Self {
        Monke {
            items: [None; ITEMS],
            operation: Operation::MulSelf,
            test: 0,
            positive: 0,
            negative: 0,
            inspected: 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Operation {
    Mul(u64),
    Sum(u64),
    MulSelf,
}

const fn get_num_monke() -> usize {
    let mut o = 1;
    let mut i = 0;
    while i < INPUT.len() - 1 {
        if INPUT[i] == b'\n' && INPUT[i + 1] == b'\n' {
            o += 1;
        }
        i += 1;
    }
    o
}

const fn get_monke() -> [Monke; NUM_MONKE] {
    let mut o = [Monke::new(); NUM_MONKE];
    let mut o_i = 0;
    let mut i = 0;
    while i < INPUT.len() {
        // skip first line
        while INPUT[i] != b'\n' {
            i += 1;
        }
        i += 19; //  Starting items:
        let mut n_i = 0;
        loop {
            let temp = [INPUT[i], INPUT[i + 1]];
            o[o_i].items[n_i] = Some(get_u64(temp));
            n_i += 1;
            if INPUT[i + 2] == b',' {
                i += 4;
            } else {
                break;
            }
        }
        while INPUT[i] != b'\n' {
            i += 1;
        }
        i += 24; //  Operation: new = old
        match (INPUT[i], INPUT[i + 2]) {
            (b'*', b'o') => o[o_i].operation = Operation::MulSelf,
            (b'*', n) => o[o_i].operation = Operation::Mul((n - b'0') as u64),
            (b'+', n) => o[o_i].operation = Operation::Sum((n - b'0') as u64),
            _ => unreachable!(),
        }
        while INPUT[i] != b'\n' {
            i += 1;
        }
        i += 22; //  Test: divisible by
        o[o_i].test = get_u64([INPUT[i], INPUT[i + 1]]);
        while INPUT[i] != b'\n' {
            i += 1;
        }
        i += 30; //    If true: throw to monkey
        o[o_i].positive = get_u64([b'0', INPUT[i]]) as usize;
        while INPUT[i] != b'\n' {
            i += 1;
        }
        i += 31; //    If false: throw to monkey
        o[o_i].negative = get_u64([b'0', INPUT[i]]) as usize;
        while INPUT[i] != b'\n' {
            i += 1;
        }
        i += 2;
        o_i += 1;
    }
    o
}

const fn get_u64(input: [u8; 2]) -> u64 {
    if input[1] >= b'0' && input[1] <= b'9' {
        (input[1] - b'0') as u64 + ((input[0] - b'0') * 10) as u64
    } else {
        (input[0] - b'0') as u64
    }
}

const fn first() -> u64 {
    let mut monkeys = MONKE;
    let mut n = 0;
    while n < 20 {
        let mut i = 0;
        while i < NUM_MONKE {
            let temp = monkeys[i].items;
            monkeys[i].items = [None; ITEMS];
            let mut t = 0;
            while t < temp.len() {
                if let Some(worry) = temp[t] {
                    let worry = match monkeys[i].operation {
                        Operation::Sum(x) => worry + x,
                        Operation::Mul(x) => worry * x,
                        Operation::MulSelf => worry * worry,
                    } / 3;
                    let monkey = if worry % monkeys[i].test == 0 {
                        monkeys[i].positive
                    } else {
                        monkeys[i].negative
                    };
                    let mut n = 0;
                    while n < ITEMS && monkeys[monkey].items[n].is_some() {
                        n += 1;
                    }
                    monkeys[monkey].items[n] = Some(worry);
                    monkeys[i].inspected += 1;
                    t += 1;
                } else {
                    break;
                }
            }
            i += 1;
        }
        n += 1;
    }
    let mut top2 = [0, 0];
    let mut i = 0;
    while i < NUM_MONKE {
        if top2[0] < monkeys[i].inspected {
            top2[1] = top2[0];
            top2[0] = monkeys[i].inspected;
        } else if top2[1] < monkeys[i].inspected {
            top2[1] = monkeys[i].inspected;
        }
        i += 1;
    }
    top2[0] * top2[1]
}

const fn second() -> u64 {
    let mut monkeys = MONKE;
    let mut mcm = 1;
    let mut i = 0;
    while i < NUM_MONKE {
        mcm *= monkeys[i].test;
        i += 1;
    }
    let mut n = 0;
    while n < 10000 {
        let mut i = 0;
        while i < NUM_MONKE {
            let temp = monkeys[i].items;
            monkeys[i].items = [None; ITEMS];
            let mut t = 0;
            while t < temp.len() {
                if let Some(worry) = temp[t] {
                    let worry = match monkeys[i].operation {
                        Operation::Sum(x) => worry + x,
                        Operation::Mul(x) => worry * x,
                        Operation::MulSelf => worry * worry,
                    } % mcm;
                    let monkey = if worry % monkeys[i].test == 0 {
                        monkeys[i].positive
                    } else {
                        monkeys[i].negative
                    };
                    let mut n = 0;
                    while n < ITEMS && monkeys[monkey].items[n].is_some() {
                        n += 1;
                    }
                    monkeys[monkey].items[n] = Some(worry);
                    monkeys[i].inspected += 1;
                    t += 1;
                } else {
                    break;
                }
            }
            i += 1;
        }
        n += 1;
    }
    let mut top2 = [0, 0];
    let mut i = 0;
    while i < NUM_MONKE {
        if top2[0] < monkeys[i].inspected {
            top2[1] = top2[0];
            top2[0] = monkeys[i].inspected;
        } else if top2[1] < monkeys[i].inspected {
            top2[1] = monkeys[i].inspected;
        }
        i += 1;
    }
    top2[0] * top2[1]
}

fn main() {
    println!("{FIRST}");
    println!("{SECOND}");
}

#[cfg(test)]
mod tests {
    #[derive(Clone, Debug)]
    struct Monkey {
        items: Vec<u64>,
        operation: Operation,
        test: u64,
        positive: usize,
        negative: usize,
        inspected: u64,
    }

    #[derive(Copy, Clone, Debug)]
    enum Operation {
        Mul(u64),
        Sum(u64),
        MulSelf,
    }

    impl<'a> From<&'a str> for Operation {
        fn from(s: &'a str) -> Self {
            match s {
                "old * 3" => Operation::Mul(3),
                "old + 2" => Operation::Sum(2),
                "old + 1" => Operation::Sum(1),
                "old + 5" => Operation::Sum(5),
                "old + 4" => Operation::Sum(4),
                "old + 8" => Operation::Sum(8),
                "old * 7" => Operation::Mul(7),
                "old * old" => Operation::MulSelf,
                _ => unreachable!(),
            }
        }
    }

    impl std::ops::Add<u64> for Operation {
        type Output = u64;
        fn add(self, rhs: u64) -> Self::Output {
            match self {
                Operation::Mul(i) => rhs * i,
                Operation::Sum(i) => rhs + i,
                Operation::MulSelf => rhs * rhs,
            }
        }
    }

    fn get_monkeys() -> Vec<Monkey> {
        let input = include_str!("../input");
        let mut monkeys = Vec::new();
        let mut lines = input.lines();
        for _ in 0..8 {
            let s = lines.next().unwrap();
            assert!(s.starts_with("Monkey "));
            let s = lines.next().unwrap();
            assert!(s.starts_with("  Starting items: "));
            let items = s[18..].split(", ").map(|s| s.parse().unwrap()).collect();
            let s = lines.next().unwrap();
            assert!(s.starts_with("  Operation: new = old "));
            let operation = Operation::from(&s[19..]);
            let s = lines.next().unwrap();
            assert!(s.starts_with("  Test: divisible by "));
            let test = s[21..].parse().unwrap();
            let s = lines.next().unwrap();
            assert!(s.starts_with("    If true: throw to monkey "));
            let positive = s[29..].parse().unwrap();
            let s = lines.next().unwrap();
            assert!(s.starts_with("    If false: throw to monkey "));
            let negative = s[30..].parse().unwrap();
            monkeys.push(Monkey {
                items,
                operation,
                test,
                positive,
                negative,
                inspected: 0,
            });
            let s = lines.next().unwrap_or_default();
            assert!(s.is_empty());
        }
        monkeys
    }
    #[test]
    fn first() {
        let mut monkeys = get_monkeys();
        for _ in 0..20 {
            for i in 0..8 {
                let temp = std::mem::take(&mut monkeys[i].items);
                for mut worry in temp {
                    worry = (monkeys[i].operation + worry) / 3;
                    let monkey = if worry % monkeys[i].test == 0 {
                        monkeys[i].positive
                    } else {
                        monkeys[i].negative
                    };
                    monkeys[monkey].items.push(worry);
                    monkeys[i].inspected += 1;
                }
            }
        }
        monkeys.sort_unstable_by(|a, b| a.inspected.cmp(&b.inspected));
        assert_eq!(monkeys[6].inspected * monkeys[7].inspected, super::FIRST); // 110220
    }
    #[test]
    fn second() {
        let mut monkeys = get_monkeys();
        for _ in 0..10000 {
            for i in 0..8 {
                let temp = std::mem::take(&mut monkeys[i].items);
                for mut worry in temp {
                    worry = (monkeys[i].operation + worry)
                        % monkeys.iter().map(|m| m.test).product::<u64>();
                    let monkey = if worry % monkeys[i].test == 0 {
                        monkeys[i].positive
                    } else {
                        monkeys[i].negative
                    };
                    monkeys[monkey].items.push(worry);
                    monkeys[i].inspected += 1;
                }
            }
        }
        monkeys.sort_unstable_by(|a, b| a.inspected.cmp(&b.inspected));
        assert_eq!(monkeys[6].inspected * monkeys[7].inspected, super::SECOND); // 19457438264
    }
}
