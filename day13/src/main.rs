use std::cmp::Ordering;

use serde::Deserialize;

#[derive(Clone, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
enum ArrayOrInt {
    Nested(Vec<ArrayOrInt>),
    Int(u8),
}

fn main() {
    let input = include_str!("../input");
    let mut lines = input
        .lines()
        .flat_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(serde_json::from_str::<ArrayOrInt>(s).unwrap())
            }
        })
        .collect::<Vec<_>>();
    let count = lines
        .chunks(2)
        .enumerate()
        .filter_map(|(i, a)| (a[0] < a[1]).then_some(i + 1))
        .sum::<usize>();
    println!("{count}"); // 6656
    lines.push(ArrayOrInt::Nested(vec![ArrayOrInt::Nested(vec![
        ArrayOrInt::Int(2),
    ])]));
    lines.push(ArrayOrInt::Nested(vec![ArrayOrInt::Nested(vec![
        ArrayOrInt::Int(6),
    ])]));
    lines.sort_unstable();
    let first = lines
        .iter()
        .position(|el| {
            el == &ArrayOrInt::Nested(vec![ArrayOrInt::Nested(vec![ArrayOrInt::Int(2)])])
        })
        .unwrap()
        + 1;
    let second = lines
        .iter()
        .position(|el| {
            el == &ArrayOrInt::Nested(vec![ArrayOrInt::Nested(vec![ArrayOrInt::Int(6)])])
        })
        .unwrap()
        + 1;
    println!("{}", first * second); // 19716
}

impl PartialOrd for ArrayOrInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (ArrayOrInt::Nested(a), ArrayOrInt::Int(b)) => ArrayOrInt::Nested(a.clone())
                .partial_cmp(&ArrayOrInt::Nested(vec![ArrayOrInt::Int(*b)])),
            (ArrayOrInt::Int(a), ArrayOrInt::Nested(b)) => {
                ArrayOrInt::Nested(vec![ArrayOrInt::Int(*a)])
                    .partial_cmp(&ArrayOrInt::Nested(b.clone()))
            }
            (ArrayOrInt::Int(a), ArrayOrInt::Int(b)) => a.partial_cmp(b),
            (ArrayOrInt::Nested(a), ArrayOrInt::Nested(b)) => {
                if a.is_empty() {
                    Some(Ordering::Less)
                } else if b.is_empty() {
                    Some(Ordering::Greater)
                } else {
                    for i in 0..=(a.len().min(b.len())) {
                        let cmp = a.get(i).partial_cmp(&b.get(i)).unwrap();
                        if !cmp.is_eq() {
                            return Some(cmp);
                        }
                    }
                    Some(Ordering::Equal)
                }
            }
        }
    }
}

impl Ord for ArrayOrInt {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn cmp() {
        let a = serde_json::from_str::<super::ArrayOrInt>("[1,1,3,1,1]").unwrap();
        let b = serde_json::from_str::<super::ArrayOrInt>("[1,1,5,1,1]").unwrap();
        assert!(a < b);

        let a = serde_json::from_str::<super::ArrayOrInt>("[[1],[2,3,4]]").unwrap();
        let b = serde_json::from_str::<super::ArrayOrInt>("[[1],4]").unwrap();
        assert!(a < b);

        let a = serde_json::from_str::<super::ArrayOrInt>("[9]").unwrap();
        let b = serde_json::from_str::<super::ArrayOrInt>("[[8,7,6]]").unwrap();
        assert!(a > b);

        let a = serde_json::from_str::<super::ArrayOrInt>("[[4,4],4,4]").unwrap();
        let b = serde_json::from_str::<super::ArrayOrInt>("[[4,4],4,4,4]").unwrap();
        assert!(a < b);

        let a = serde_json::from_str::<super::ArrayOrInt>("[7,7,7,7]").unwrap();
        let b = serde_json::from_str::<super::ArrayOrInt>("[7,7,7]").unwrap();
        assert!(a > b);

        let a = serde_json::from_str::<super::ArrayOrInt>("[]").unwrap();
        let b = serde_json::from_str::<super::ArrayOrInt>("[3]").unwrap();
        assert!(a < b);

        let a = serde_json::from_str::<super::ArrayOrInt>("[[[]]]").unwrap();
        let b = serde_json::from_str::<super::ArrayOrInt>("[[]]").unwrap();
        assert!(a > b);

        let a = serde_json::from_str::<super::ArrayOrInt>("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap();
        let b = serde_json::from_str::<super::ArrayOrInt>("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap();
        assert!(a > b);
    }
}
