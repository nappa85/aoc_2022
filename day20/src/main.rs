fn mix(array: &mut Vec<(usize, i64)>) {
    let len = array.len() as i64 - 1;
    for i in 0..array.len() {
        let pos = array.iter().position(|(j, _)| j == &i).unwrap();
        let element = array.remove(pos);
        array.insert((pos as i64 + element.1).rem_euclid(len) as usize, element);
    }
}

fn main() {
    let mut input = include_str!("../input")
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .enumerate()
        .collect::<Vec<_>>();

    let mut temp = input.clone();
    mix(&mut temp);
    let pos = temp.iter().position(|(_, i)| *i == 0).unwrap();
    println!(
        "{}",
        temp[(pos + 1000) % temp.len()].1
            + temp[(pos + 2000) % temp.len()].1
            + temp[(pos + 3000) % temp.len()].1
    ); // 3346

    input.iter_mut().for_each(|(_, i)| *i *= 811589153);
    for _ in 0..10 {
        mix(&mut input);
    }
    let pos = input.iter().position(|(_, i)| *i == 0).unwrap();
    println!(
        "{}",
        input[(pos + 1000) % input.len()].1
            + input[(pos + 2000) % input.len()].1
            + input[(pos + 3000) % input.len()].1
    ); // 4265712588168
}

#[cfg(test)]
mod tests {
    #[test]
    fn first() {
        let mut input = include_str!("../example")
            .lines()
            .map(|s| s.parse::<i64>().unwrap())
            .enumerate()
            .collect::<Vec<_>>();
        super::mix(&mut input);
        // assert_eq!(input, [1, 2, -3, 4, 0, 3, -2]);
        let pos = input.iter().position(|(_, i)| *i == 0).unwrap();
        assert_eq!(
            input[(pos + 1000) % input.len()].1
                + input[(pos + 2000) % input.len()].1
                + input[(pos + 3000) % input.len()].1,
            3
        )
    }

    #[test]
    fn second() {
        let mut input = include_str!("../example")
            .lines()
            .map(|s| s.parse::<i64>().unwrap() * 811589153)
            .enumerate()
            .collect::<Vec<_>>();
        for _ in 0..10 {
            super::mix(&mut input);
        }
        // assert_eq!(
        //     &[
        //         811589153,
        //         1623178306,
        //         -2434767459,
        //         2434767459,
        //         -1623178306,
        //         0,
        //         3246356612
        //     ][..],
        //     input
        // );
        let pos = input.iter().position(|(_, i)| *i == 0).unwrap();
        assert_eq!(
            input[(pos + 1000) % input.len()].1
                + input[(pos + 2000) % input.len()].1
                + input[(pos + 3000) % input.len()].1,
            1623178306
        )
    }
}
