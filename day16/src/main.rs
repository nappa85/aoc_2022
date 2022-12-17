use std::collections::HashMap;

use itertools::Itertools;

use rayon::prelude::{ParallelBridge, ParallelIterator};

#[derive(Debug)]
struct Valve<'a> {
    name: &'a str,
    rate: u8,
    leads_to: Vec<&'a str>,
}

// fn next_valve<'a>(
//     valves: &'a HashMap<&'a str, Valve>,
//     minutes: u8,
//     path: &[(&'a str, u8)],
// ) -> Option<(&'a str, u8)> {
//     let actual = path.last().unwrap();
//     let mut v = valves
//         .iter()
//         .filter_map(|(name, valve)| {
//             if path.iter().filter(|(n, v)| name == n && *v > 0).count() == 0 {
//                 let rate = valve.rate as i16
//                     * (minutes as i16 - get_distance(valves, actual.0, name) as i16);
//                 println!(
//                     "minute {} distance * rate {} between {} and {} = {}",
//                     minutes, valve.rate, actual.0, name, rate,
//                 );
//                 (rate > 0).then_some((*name, rate))
//             } else {
//                 None
//             }
//         })
//         .collect::<Vec<_>>();
//     if v.is_empty() {
//         return None;
//     }
//     v.sort_unstable_by(|a, b| a.1.cmp(&b.1));
//     println!("{v:?}");
//     let target = v.last().unwrap().0;
//     if actual.0 == target {
//         return Some((target, valves[target].rate));
//     }
//     let next = get_next_step(valves, actual.0, target);
//     Some((next, 0))
// }

fn get_distance<'a>(valves: &'a HashMap<&'a str, Valve>, start: &'a str, end: &'a str) -> u8 {
    let mut i = 0;
    let mut visited = vec![vec![start]];
    loop {
        let last = visited.last().unwrap();
        if last.contains(&end) {
            return i;
        }
        let next = last
            .iter()
            .flat_map(|v| valves[v].leads_to.iter().copied())
            .collect::<Vec<_>>();
        visited.push(next);
        i += 1;
    }
}

// fn get_next_step<'a>(valves: &'a HashMap<&'a str, Valve>, start: &'a str, end: &'a str) -> &'a str {
//     let mut v = valves[start]
//         .leads_to
//         .iter()
//         .map(|n| (n, get_distance(valves, n, end)))
//         .collect::<Vec<_>>();
//     v.sort_unstable_by(|a, b| a.1.cmp(&b.1));
//     v.first().unwrap().0
// }

fn main() {
    let input = include_str!("../input");
    // let input = include_str!("../example");
    let valves = input
        .lines()
        .map(|s| {
            let mut temp = s.split(' ');
            let name = temp.nth(1).unwrap();
            let rate = temp.nth(2).unwrap();
            let rate = rate[5..(rate.len() - 1)].parse().unwrap();
            let leads_to = temp
                .skip(4)
                .map(|s| s.trim_matches(|c: char| !c.is_alphabetic()))
                .collect();
            (
                name,
                Valve {
                    name,
                    rate,
                    leads_to,
                },
            )
        })
        .collect::<HashMap<_, _>>();
    let distances = valves
        .keys()
        .map(|start| {
            (
                *start,
                valves
                    .iter()
                    .filter(|(n, v)| v.rate > 0 && *n != start)
                    .map(|(end, _)| (*end, get_distance(&valves, start, end) + 1)) // + 1 is the activation cost
                    .collect::<HashMap<_, _>>(),
            )
        })
        .collect::<HashMap<_, _>>();

    let mut perms = valves
        .iter()
        .filter(|(_, v)| v.rate > 0)
        .map(|(n, _)| *n)
        .permutations(distances["AA"].len().min(7)) // after 7 it become computationally infinite
        .par_bridge()
        .filter(|perm| {
            perm.iter()
                .fold(("AA", 0), |(prev, sum), next| {
                    (*next, sum + distances[prev][*next])
                })
                .1
                <= 30
        })
        .map(|perm| {
            let temp = perm
                .iter()
                .fold(("AA", 30, 0), |(prev, minutes, pressure), next| {
                    let remaining = minutes - distances[prev][*next];
                    (
                        *next,
                        remaining,
                        pressure + valves[*next].rate as u16 * remaining as u16,
                    )
                });
            (perm, temp.2)
        })
        .collect::<Vec<_>>();
    perms.sort_unstable_by(|a, b| a.1.cmp(&b.1));
    println!("{:?}", perms.last().unwrap()); // 1584

    for i in 2..=(distances["AA"].len()) {
        let mut perms = valves
            .iter()
            .filter(|(_, v)| v.rate > 0)
            .map(|(n, _)| *n)
            .permutations(i)
            .par_bridge()
            .filter(|perm| {
                perm.iter()
                    .take(perm.len() / 2)
                    .fold(("AA", 0), |(prev, sum), next| {
                        (*next, sum + distances[prev][*next])
                    })
                    .1
                    <= 26
                    && perm
                        .iter()
                        .skip(perm.len() / 2)
                        .fold(("AA", 0), |(prev, sum), next| {
                            (*next, sum + distances[prev][*next])
                        })
                        .1
                        <= 26
            })
            .map(|perm| {
                let temp1 = perm.iter().take(perm.len() / 2).fold(
                    ("AA", 26, 0),
                    |(prev, minutes, pressure), next| {
                        let remaining = minutes - distances[prev][*next];
                        (
                            *next,
                            remaining,
                            pressure + valves[*next].rate as u16 * remaining as u16,
                        )
                    },
                );
                let temp2 = perm.iter().skip(perm.len() / 2).fold(
                    ("AA", 26, 0),
                    |(prev, minutes, pressure), next| {
                        let remaining = minutes - distances[prev][*next];
                        (
                            *next,
                            remaining,
                            pressure + valves[*next].rate as u16 * remaining as u16,
                        )
                    },
                );
                (perm, temp1.2 + temp2.2)
            })
            .collect::<Vec<_>>();
        perms.sort_unstable_by(|a, b| a.1.cmp(&b.1));
        println!("{:?}", perms.last().unwrap()); //
    }
}
