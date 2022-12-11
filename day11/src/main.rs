
fn main() {
    let input = "Monkey 0:
  Starting items: 53, 89, 62, 57, 74, 51, 83, 97
  Operation: new = old * 3
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 5

Monkey 1:
  Starting items: 85, 94, 97, 92, 56
  Operation: new = old + 2
  Test: divisible by 19
    If true: throw to monkey 5
    If false: throw to monkey 2

Monkey 2:
  Starting items: 86, 82, 82
  Operation: new = old + 1
  Test: divisible by 11
    If true: throw to monkey 3
    If false: throw to monkey 4

Monkey 3:
  Starting items: 94, 68
  Operation: new = old + 5
  Test: divisible by 17
    If true: throw to monkey 7
    If false: throw to monkey 6

Monkey 4:
  Starting items: 83, 62, 74, 58, 96, 68, 85
  Operation: new = old + 4
  Test: divisible by 3
    If true: throw to monkey 3
    If false: throw to monkey 6

Monkey 5:
  Starting items: 50, 68, 95, 82
  Operation: new = old + 8
  Test: divisible by 7
    If true: throw to monkey 2
    If false: throw to monkey 4

Monkey 6:
  Starting items: 75
  Operation: new = old * 7
  Test: divisible by 5
    If true: throw to monkey 7
    If false: throw to monkey 0

Monkey 7:
  Starting items: 92, 52, 85, 89, 68, 82
  Operation: new = old * old
  Test: divisible by 2
    If true: throw to monkey 0
    If false: throw to monkey 1
";
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
        monkeys.push(Monkey { items, operation, test, positive, negative, inspected: 0 });
        let s = lines.next().unwrap_or_default();
        assert!(s.is_empty());
    }
    let backup = monkeys.clone();
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
    println!("{}", monkeys[6].inspected * monkeys[7].inspected);
    let mut monkeys = backup;
    for _ in 0..10000 {
        for i in 0..8 {
            let temp = std::mem::take(&mut monkeys[i].items);
            for mut worry in temp {
                worry = (monkeys[i].operation + worry) % monkeys.iter().map(|m| m.test).product::<u64>();
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
    println!("{}", monkeys[6].inspected * monkeys[7].inspected);
}

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
