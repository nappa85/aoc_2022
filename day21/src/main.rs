use std::collections::HashMap;

#[derive(Debug)]
enum Operation<'a> {
    Number(i64),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
}

impl<'a> Operation<'a> {
    fn solve(&self, monke: &HashMap<&'a str, Operation<'a>>) -> i64 {
        match self {
            Operation::Number(n) => *n,
            Operation::Add(a, b) => monke[a].solve(monke) + monke[b].solve(monke),
            Operation::Sub(a, b) => monke[a].solve(monke) - monke[b].solve(monke),
            Operation::Mul(a, b) => monke[a].solve(monke) * monke[b].solve(monke),
            Operation::Div(a, b) => monke[a].solve(monke) / monke[b].solve(monke),
        }
    }
    fn counter_solve(&self, val: i64, monke: &HashMap<&'a str, Operation<'a>>) -> i64 {
        println!("counter_solve {self:?} {val}");
        match self {
            Operation::Number(n) => *n,
            // a + b = c => a = c - b // b = c - a
            Operation::Add("humn", ab) | Operation::Add(ab, "humn") => val - monke[ab].solve(monke),
            Operation::Add(a, b) if monke[a].contains_humn(monke) => {
                monke[a].counter_solve(val - monke[b].solve(monke), monke)
            }
            Operation::Add(a, b) if monke[b].contains_humn(monke) => {
                monke[b].counter_solve(val - monke[a].solve(monke), monke)
            }
            // a - b = c => a = c + b // b  = a - c
            Operation::Sub("humn", b) => val + monke[b].solve(monke),
            Operation::Sub(a, "humn") => monke[a].solve(monke) - val,
            Operation::Sub(a, b) if monke[a].contains_humn(monke) => {
                monke[a].counter_solve(val + monke[b].solve(monke), monke)
            }
            Operation::Sub(a, b) if monke[b].contains_humn(monke) => {
                monke[b].counter_solve(monke[a].solve(monke) - val, monke)
            }
            // a * b = c => a = c / b // b = c / a
            Operation::Mul("humn", ab) | Operation::Mul(ab, "humn") => val / monke[ab].solve(monke),
            Operation::Mul(a, b) if monke[a].contains_humn(monke) => {
                monke[a].counter_solve(val / monke[b].solve(monke), monke)
            }
            Operation::Mul(a, b) if monke[b].contains_humn(monke) => {
                monke[b].counter_solve(val / monke[a].solve(monke), monke)
            }
            // a / b = c => a = c * b // c = a / c
            Operation::Div("humn", b) => val * monke[b].solve(monke),
            Operation::Div(a, "humn") => monke[a].solve(monke) / val,
            Operation::Div(a, b) if monke[a].contains_humn(monke) => {
                monke[a].counter_solve(val * monke[b].solve(monke), monke)
            }
            Operation::Div(a, b) if monke[b].contains_humn(monke) => {
                monke[b].counter_solve(monke[a].solve(monke) / val, monke)
            }
            _ => unreachable!(),
        }
    }
    fn contains_humn(&self, monke: &HashMap<&'a str, Operation<'a>>) -> bool {
        match self {
            Operation::Number(_) => false,
            Operation::Add("humn", _) | Operation::Add(_, "humn") => true,
            Operation::Add(a, b) => monke[a].contains_humn(monke) || monke[b].contains_humn(monke),
            Operation::Sub("humn", _) | Operation::Sub(_, "humn") => true,
            Operation::Sub(a, b) => monke[a].contains_humn(monke) || monke[b].contains_humn(monke),
            Operation::Mul("humn", _) | Operation::Mul(_, "humn") => true,
            Operation::Mul(a, b) => monke[a].contains_humn(monke) || monke[b].contains_humn(monke),
            Operation::Div("humn", _) | Operation::Div(_, "humn") => true,
            Operation::Div(a, b) => monke[a].contains_humn(monke) || monke[b].contains_humn(monke),
        }
    }
}

impl<'a> From<&[&'a str]> for Operation<'a> {
    fn from(value: &[&'a str]) -> Self {
        match value.len() {
            1 => Operation::Number(value[0].parse().unwrap()),
            3 => match value[1] {
                "+" => Operation::Add(value[0], value[2]),
                "-" => Operation::Sub(value[0], value[2]),
                "*" => Operation::Mul(value[0], value[2]),
                "/" => Operation::Div(value[0], value[2]),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

fn parse(input: &'_ str) -> HashMap<&'_ str, Operation<'_>> {
    input
        .lines()
        .map(|s| {
            let mut temp = s.split(' ');
            let name = temp.next().unwrap();
            let parts = temp.collect::<Vec<_>>();
            (&name[..(name.len() - 1)], Operation::from(parts.as_ref()))
        })
        .collect()
}

fn second(monke: HashMap<&'_ str, Operation<'_>>) -> i64 {
    let (a, b) = match monke["root"] {
        Operation::Add(a, b) => (a, b),
        Operation::Sub(a, b) => (a, b),
        Operation::Mul(a, b) => (a, b),
        Operation::Div(a, b) => (a, b),
        _ => unreachable!(),
    };
    match (
        monke[a].contains_humn(&monke),
        monke[b].contains_humn(&monke),
    ) {
        (true, false) => {
            let val = monke[b].solve(&monke);
            monke[a].counter_solve(val, &monke)
        }
        (false, true) => {
            let val = monke[a].solve(&monke);
            monke[b].counter_solve(val, &monke)
        }
        _ => unreachable!(),
    }
}

fn main() {
    let input = include_str!("../input");
    let monke = parse(input);
    println!("{}", monke["root"].solve(&monke)); // 83056452926300

    println!("{}", second(monke)); // 3469704905529
}

#[cfg(test)]
mod tests {
    #[test]
    fn first() {
        let input = include_str!("../example");
        let monke = super::parse(input);
        assert_eq!(monke["root"].solve(&monke), 152);
    }
    #[test]
    fn second() {
        let input = include_str!("../example");
        let monke = super::parse(input);
        assert_eq!(super::second(monke), 301);
    }
}
