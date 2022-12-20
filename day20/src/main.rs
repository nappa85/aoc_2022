fn shift(input: &str) -> Vec<i16> {
    let mut input = input
        .lines()
        .map(|s| (s.parse::<i16>().unwrap(), true))
        .collect::<Vec<_>>();
    let len = input.len() as i16;
    while let Some(pos) = input.iter().position(|(_, b)| *b) {
        let (element, _) = input.remove(pos);
        let mut destination = pos as i16 + element;
        if element < 0 {
            destination -= 1;
        }
        while destination < 0 {
            destination += len;
        }
        // println!(
        //     "{element} moves between {} and {}:",
        //     input[((destination - 1) % (len - 1)) as usize].0,
        //     input[(destination % (len - 1)) as usize].0
        // );
        if destination % len == len - 1 {
            input.push((element, false));
        } else {
            input.insert((destination % (len - 1)) as usize, (element, false));
        }
        // println!("{:?}", input.iter().map(|(e, _)| e).collect::<Vec<_>>());
    }
    input.into_iter().map(|(e, _)| e).collect()
}

fn main() {
    let input = include_str!("../input");
    let input = shift(input);
    let pos = input.iter().position(|i| *i == 0).unwrap();
    println!(
        "{}",
        dbg!(input[(pos + 1000) % input.len()])
            + dbg!(input[(pos + 2000) % input.len()])
            + dbg!(input[(pos + 3000) % input.len()])
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn first() {
        let input = include_str!("../example");
        let input = super::shift(input);
        assert_eq!(input, [1, 2, -3, 4, 0, 3, -2]);
        let pos = input.iter().position(|i| *i == 0).unwrap();
        assert_eq!(
            3,
            input[(pos + 1000) % input.len()]
                + input[(pos + 2000) % input.len()]
                + input[(pos + 3000) % input.len()]
        )
    }
}
