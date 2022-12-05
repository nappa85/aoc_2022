use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input").lines();
    let mut crates = input.clone().take_while(|s| !s.is_empty()).fold(
        vec![VecDeque::new(); 9],
        |mut piles, s| {
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
        },
    );

    let moves = input
        .clone()
        .skip_while(|s| !s.is_empty())
        .skip(1)
        .map(|s| {
            let parts = s.split(" ");
            let take = parts
                .clone()
                .skip(1)
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let from = parts
                .clone()
                .skip(3)
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let to = parts.skip(5).next().unwrap().parse::<usize>().unwrap();
            (take, from, to)
        })
        .collect::<Vec<_>>();

    let backup = crates.clone();
    moves.iter().for_each(|(take, from, to)| {
        for _ in 0..*take {
            let temp = crates[from - 1].pop_back().unwrap();
            crates[to - 1].push_back(temp);
        }
    });
    for c in &crates {
        print!("{:?}", c.back().unwrap());
    }
    print!("\n");

    let mut crates = backup;
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
    for c in &crates {
        print!("{:?}", c.back().unwrap());
    }
    print!("\n");
}
