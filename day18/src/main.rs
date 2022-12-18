use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    // let input = include_str!("../example");
    let cubes = input
        .lines()
        .map(|s| {
            let temp = s
                .split(',')
                .map(|i| i.parse::<i8>().unwrap())
                .collect::<Vec<_>>();
            (temp[0], temp[1], temp[2])
        })
        .collect::<Vec<_>>();
    let sum = cubes
        .iter()
        .map(|c1| {
            let x = cubes
                .iter()
                .filter(|c2| {
                    ((c1.0 - 1)..=(c1.0 + 1)).contains(&c2.0) && c1.1 == c2.1 && c1.2 == c2.2
                })
                .count()
                - 1;
            let y = cubes
                .iter()
                .filter(|c2| {
                    c1.0 == c2.0 && ((c1.1 - 1)..=(c1.1 + 1)).contains(&c2.1) && c1.2 == c2.2
                })
                .count()
                - 1;
            let z = cubes
                .iter()
                .filter(|c2| {
                    c1.1 == c2.1 && c1.0 == c2.0 && ((c1.2 - 1)..=(c1.2 + 1)).contains(&c2.2)
                })
                .count()
                - 1;

            6 - x - y - z
        })
        .sum::<usize>();
    println!("{sum}"); // 3636

    let x = cubes.iter().fold(HashMap::new(), |mut acc, c| {
        let entry = acc.entry((c.1, c.2)).or_insert((i8::MAX, i8::MIN));
        entry.0 = entry.0.min(c.0);
        entry.1 = entry.1.max(c.0);
        acc
    });
    let y = cubes.iter().fold(HashMap::new(), |mut acc, c| {
        let entry = acc.entry((c.0, c.2)).or_insert((i8::MAX, i8::MIN));
        entry.0 = entry.0.min(c.1);
        entry.1 = entry.1.max(c.1);
        acc
    });
    let z = cubes.iter().fold(HashMap::new(), |mut acc, c| {
        let entry = acc.entry((c.0, c.1)).or_insert((i8::MAX, i8::MIN));
        entry.0 = entry.0.min(c.2);
        entry.1 = entry.1.max(c.2);
        acc
    });

    let min_x = x.values().map(|(min, _)| *min).min().unwrap();
    let max_x = x.values().map(|(_, max)| *max).max().unwrap();
    let min_y = y.values().map(|(min, _)| *min).min().unwrap();
    let max_y = y.values().map(|(_, max)| *max).max().unwrap();
    let min_z = z.values().map(|(min, _)| *min).min().unwrap();
    let max_z = z.values().map(|(_, max)| *max).max().unwrap();

    let mut solid = (min_x..=max_x)
        .flat_map(move |x| {
            (min_y..=max_y).flat_map(move |y| (min_z..=max_z).map(move |z| (x, y, z)))
        })
        .collect::<Vec<_>>();

    x.iter().for_each(|((y, z), (min, max))| {
        for x in (min_x..*min).chain((*max + 1)..=max_x) {
            for y in (min_y..=*y).rev() {
                if !cubes.contains(&(x, y, *z)) {
                    solid.retain(|c| c != &(x, y, *z));
                } else {
                    break;
                }
            }
            for y in *y..=max_y {
                if !cubes.contains(&(x, y, *z)) {
                    solid.retain(|c| c != &(x, y, *z));
                } else {
                    break;
                }
            }
            for z in (min_z..=*z).rev() {
                if !cubes.contains(&(x, *y, z)) {
                    solid.retain(|c| c != &(x, *y, z));
                } else {
                    break;
                }
            }
            for z in *z..=max_z {
                if !cubes.contains(&(x, *y, z)) {
                    solid.retain(|c| c != &(x, *y, z));
                } else {
                    break;
                }
            }
        }
    });
    y.iter().for_each(|((x, z), (min, max))| {
        for y in (min_y..*min).chain((*max + 1)..=max_y) {
            for x in (min_x..=*x).rev() {
                if !cubes.contains(&(x, y, *z)) {
                    solid.retain(|c| c != &(x, y, *z));
                } else {
                    break;
                }
            }
            for x in *x..=max_x {
                if !cubes.contains(&(x, y, *z)) {
                    solid.retain(|c| c != &(x, y, *z));
                } else {
                    break;
                }
            }
            for z in (min_z..=*z).rev() {
                if !cubes.contains(&(*x, y, z)) {
                    solid.retain(|c| c != &(*x, y, z));
                } else {
                    break;
                }
            }
            for z in *z..=max_z {
                if !cubes.contains(&(*x, y, z)) {
                    solid.retain(|c| c != &(*x, y, z));
                } else {
                    break;
                }
            }
        }
    });
    z.iter().for_each(|((x, y), (min, max))| {
        for z in (min_z..*min).chain((*max + 1)..=max_z) {
            for x in (min_x..=*x).rev() {
                if !cubes.contains(&(x, *y, z)) {
                    solid.retain(|c| c != &(x, *y, z));
                } else {
                    break;
                }
            }
            for x in *x..=max_x {
                if !cubes.contains(&(x, *y, z)) {
                    solid.retain(|c| c != &(x, *y, z));
                } else {
                    break;
                }
            }
            for y in (min_y..=*y).rev() {
                if !cubes.contains(&(*x, y, z)) {
                    solid.retain(|c| c != &(*x, y, z));
                } else {
                    break;
                }
            }
            for y in *y..=max_y {
                if !cubes.contains(&(*x, y, z)) {
                    solid.retain(|c| c != &(*x, y, z));
                } else {
                    break;
                }
            }
        }
    });

    let sum = solid
        .iter()
        .map(|c1| {
            let x = solid
                .iter()
                .filter(|c2| {
                    ((c1.0 - 1)..=(c1.0 + 1)).contains(&c2.0) && c1.1 == c2.1 && c1.2 == c2.2
                })
                .count()
                - 1;
            let y = solid
                .iter()
                .filter(|c2| {
                    c1.0 == c2.0 && ((c1.1 - 1)..=(c1.1 + 1)).contains(&c2.1) && c1.2 == c2.2
                })
                .count()
                - 1;
            let z = solid
                .iter()
                .filter(|c2| {
                    c1.1 == c2.1 && c1.0 == c2.0 && ((c1.2 - 1)..=(c1.2 + 1)).contains(&c2.2)
                })
                .count()
                - 1;

            6 - x - y - z
        })
        .sum::<usize>();
    println!("{sum}"); // 2102
}
