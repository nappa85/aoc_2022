use std::collections::HashMap;

const TARGET: i64 = 2000000;
// const TARGET: i64 = 10;

#[derive(Copy, Clone, Debug)]
enum Things {
    None,
    Sensor,
    Beacon,
    Area,
}

impl Things {
    fn is_area(&self) -> bool {
        matches!(self, Things::Area)
    }
}

impl<'a> From<Option<&'a Things>> for Things {
    fn from(t: Option<&'a Things>) -> Self {
        t.copied().unwrap_or(Things::None)
    }
}

fn main() {
    let input = include_str!("../input");
    // let input = include_str!("../example");
    let coords = input
        .lines()
        .map(|s| {
            let mut parts = s.split(' ');
            let sx = parts.nth(2).unwrap();
            let sx = sx[2..(sx.len() - 1)].parse::<i64>().unwrap();
            let sy = parts.next().unwrap();
            let sy = sy[2..(sy.len() - 1)].parse::<i64>().unwrap();
            let bx = parts.nth(4).unwrap();
            let bx = bx[2..(bx.len() - 1)].parse::<i64>().unwrap();
            let by = parts.next().unwrap();
            let by = by[2..].parse::<i64>().unwrap();
            (sx, sy, bx, by)
        })
        .collect::<Vec<_>>();
    let (min_x, min_y, _max_x, _max_y) =
        coords.iter().fold((0, 0, 0, 0), |mut a, (sx, sy, bx, by)| {
            let bow = (sx - bx).abs() + (sy - by).abs();
            a.0 = a.0.min(*sx - bow);
            a.1 = a.1.min(*sy - bow);
            a.2 = a.2.max(*sx + bow);
            a.3 = a.3.max(*sy + bow);
            a
        });
    // let mut grid = vec![
    //     vec![Things::None; usize::try_from(max_y - min_y).unwrap() + 1];
    //     usize::try_from(max_x - min_x).unwrap() + 1
    // ];
    let mut grid = HashMap::new();
    let mut insert = |x, y, t| {
        let entry = grid.entry(x).or_insert_with(HashMap::new);
        entry.entry(y).or_insert(t);
    };
    for (sx, sy, bx, by) in &coords {
        insert(
            (*sx - min_x) as usize,
            (*sy - min_y) as usize,
            Things::Sensor,
        );
        insert(
            (*bx - min_x) as usize,
            (*by - min_y) as usize,
            Things::Beacon,
        );
        let bow = (sx - bx).abs() + (sy - by).abs();
        let coords = ((*sx - bow)..=(*sx + bow)).flat_map(|x| {
            if ((*sy - bow)..=(*sy + bow)).contains(&TARGET) {
                // ((*sy - bow)..=(*sy + bow)).flat_map(move |y| {
                if sx.max(&x) - sx.min(&x) + sy.max(&TARGET) - sy.min(&TARGET) <= bow {
                    Some(((x - min_x) as usize, (TARGET - min_y) as usize))
                } else {
                    None
                }
                // })
            } else {
                None
            }
        });
        for (x, y) in coords {
            insert(x, y, Things::Area);
        }
    }
    // for y in 0..=(_max_y - min_y) {
    //     print!("{:#07} ", y + min_y);
    //     for x in 0..=(_max_x - min_x) {
    //         match Things::from(
    //             grid.get(&(x as usize))
    //                 .and_then(|col| col.get(&(y as usize))),
    //         ) {
    //             Things::None => print!("."),
    //             Things::Sensor => print!("S"),
    //             Things::Beacon => print!("B"),
    //             Things::Area => print!("#"),
    //         }
    //     }
    //     print!("\n");
    // }
    let count = grid
        .iter()
        .filter(|(_, col)| Things::from(col.get(&((TARGET - min_y) as usize))).is_area())
        .count();
    println!("{count}"); // 4985193

    let (min_x, min_y, max_x, max_y) = coords.iter().fold((0, 0, 0, 0), |mut a, (sx, sy, _, _)| {
        a.0 = a.0.min(*sx);
        a.1 = a.1.min(*sy);
        a.2 = a.2.max(*sx);
        a.3 = a.3.max(*sy);
        a
    });
    coords
        .iter()
        .flat_map(|(sx, sy, bx, by)| {
            let bow = (sx - bx).abs() + (sy - by).abs() + 1;
            ((sx - bow)..*sx)
                .zip(*sy..(sy + bow))
                .chain((*sx..(sx + bow)).zip((*sy..(sy + bow)).rev()))
                .chain((*sx..(sx + bow)).rev().zip(((sy - bow)..*sy).rev()))
                .chain(((sx - bow)..*sx).rev().zip((sy - bow)..*sy))
        })
        .filter(|(x, y)| {
            *x >= min_x.max(0) && *x <= max_x.min(4000000) && *y >= min_y && *y <= max_y
        })
        .for_each(|(x, y)| {
            if coords.iter().all(|(sx, sy, bx, by)| {
                let bow = (sx - bx).abs() + (sy - by).abs();
                (*sx - x).abs() + (*sy - y).abs() > bow
            }) {
                println!("{}", x * 4000000 + y); // 11583882601918
            }
        });
}
