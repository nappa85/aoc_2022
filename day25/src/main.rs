fn snafu_to_int(input: &str) -> i64 {
    input
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| match c {
            '-' => -1,
            '=' => -2,
            x => x.to_digit(10).unwrap() as i64,
        } * 5_i64.pow(i as u32))
        .sum()
}

fn int_to_snafu(mut input: i64) -> String {
    let mut out = vec![];
    while input > 0 {
        let rem = input % 5;
        input /= 5;

        match rem {
            4 => {
                out.push('-');
                input += 1;
            }
            3 => {
                out.push('=');
                input += 1;
            }
            _ => out.push(char::from_digit(rem as u32, 10).unwrap()),
        }
    }
    out.into_iter().rev().collect()
}

fn main() {
    let input = include_str!("../input");
    let sum: i64 = input.lines().map(snafu_to_int).sum();
    println!("{}", int_to_snafu(sum));
}

#[cfg(test)]
mod tests {
    #[test]
    fn first() {
        let input = include_str!("../example");
        let sum: i64 = input.lines().map(super::snafu_to_int).sum();
        assert_eq!(sum, 4890);
        assert_eq!(super::int_to_snafu(sum), "2=-1=0");
    }
}
