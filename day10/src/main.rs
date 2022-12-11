enum Op {
    Noop,
    Addx(i8),
}

fn main() {
    let input = "noop
addx 12
addx -5
addx -1
noop
addx 4
noop
addx 1
addx 4
noop
addx 13
addx -8
noop
addx -19
addx 24
addx 1
noop
addx 4
noop
addx 1
addx 5
addx -1
addx -37
addx 16
addx -13
addx 18
addx -11
addx 2
addx 23
noop
addx -18
addx 9
addx -8
addx 2
addx 5
addx 2
addx -21
addx 26
noop
addx -15
addx 20
noop
addx 3
noop
addx -38
addx 3
noop
addx 26
addx -4
addx -19
addx 3
addx 1
addx 5
addx 3
noop
addx 2
addx 3
noop
addx 2
noop
noop
noop
noop
addx 5
noop
noop
noop
addx 3
noop
addx -30
addx -4
addx 1
addx 18
addx -8
addx -4
addx 2
noop
addx 7
noop
noop
noop
noop
addx 5
noop
noop
addx 5
addx -2
addx -20
addx 27
addx -20
addx 25
addx -2
addx -35
noop
noop
addx 4
addx 3
addx -2
addx 5
addx 2
addx -11
addx 1
addx 13
addx 2
addx 5
addx 6
addx -1
addx -2
noop
addx 7
addx -2
addx 6
addx 1
addx -21
addx 22
addx -38
addx 5
addx 3
addx -1
noop
noop
addx 5
addx 1
addx 4
addx 3
addx -2
addx 2
noop
addx 7
addx -1
addx 2
addx 4
addx -10
addx -19
addx 35
addx -1
noop
noop
noop
";
    let v = input.lines().map(|s| match s {
        "noop" => Op::Noop,
        _ => Op::Addx(s[5..].parse().unwrap()),
    }).fold(vec![1], |mut a, op| {
        match op {
            Op::Noop => {
                let temp = *a.last().unwrap();
                a.push(temp);
            },
            Op::Addx(n) => {
                let temp = *a.last().unwrap();
                a.push(temp);
                a.push(temp + n)
            },
        }
        a
    });
    let sum: i32 = [20, 60, 100, 140, 180, 220].into_iter().map(|i| i as i32 * v[i as usize - 1] as i32).sum();
    println!("{sum}");
    v.chunks(40).for_each(|chunk| {
        for i in 0_i8..40_i8 {
            if ((i - 1)..=(i + 1)).contains(&(chunk[i as usize])) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    })
}
