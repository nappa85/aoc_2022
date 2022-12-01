const INPUT: &[u8] = include_bytes!("../input");

const fn get_elves_count() -> usize {
    let mut index = 0;
    let mut count = 1;
    while index < INPUT.len() {
        if INPUT[index] == 10 && index > 0 && INPUT[index - 1] == 10 {
            count += 1;
        }
        index += 1;
    }
    count
}

const fn calc_value(val: [Option<u8>; 10]) -> u32 {
    let mut index = 9;
    let mut power = 0;
    let mut tot = 0;
    loop {
        if let Some(v) = val[index] {
            tot += v as u32 * 10_u32.pow(power);
            power += 1;
        }
        if index == 0 {
            break;
        } else {
            index -= 1;
        }
    }
    tot
}

const fn get_elves() -> [u32; get_elves_count()] {
    let mut index = 0;
    let mut current_elf = 0;
    let mut current = [None; 10];
    let mut current_index = 0;
    let mut elves = [0; get_elves_count()];
    while index < INPUT.len() {
        if INPUT[index] == 10 && index > 0 && INPUT[index - 1] == 10 {
            current_elf += 1;
        } else if INPUT[index] == 10 {
            elves[current_elf] += calc_value(current);
            current_index = 0;
            current = [None; 10];
        } else {
            current[current_index] = Some(INPUT[index] - 48);
            current_index += 1;
        }
        index += 1;
    }
    elves
}

const fn get_max_elf() -> u32 {
    let mut max = 0;
    let mut index = 0;
    let elves = get_elves();
    while index < elves.len() {
        if max < elves[index] {
            max = elves[index];
        }
        index += 1;
    }
    max
}

const fn get_top3_elfs() -> [u32; 3] {
    let mut max = [0; 3];
    let mut index = 0;
    let elves = get_elves();
    while index < elves.len() {
        if max[0] < elves[index] {
            max[2] = max[1];
            max[1] = max[0];
            max[0] = elves[index];
        } else if max[1] < elves[index] {
            max[2] = max[1];
            max[1] = elves[index];
        } else if max[2] < elves[index] {
            max[2] = elves[index];
        }
        index += 1;
    }
    max
}

const fn sum_top3_elfs() -> u32 {
    let elfs = get_top3_elfs();
    elfs[0] + elfs[1] + elfs[2]
}

fn main() {
    println!("Max elf has {} calories", get_max_elf());
    println!("Top 3 elf: {:#?}", get_top3_elfs());
    println!("Sum top 3 elf: {:#?} calories", sum_top3_elfs());
}

#[cfg(test)]
mod tests {
    fn get_elves() -> Vec<u32> {
        let s = include_str!("../input");
        let elves = s.lines().fold(vec![0], |mut elves, s| {
            if s.is_empty() {
                elves.push(0);
            } else {
                let current = elves.len() - 1;
                elves[current] += s.parse::<u32>().unwrap();
            }
            elves
        });
        elves
    }

    #[test]
    fn elves() {
        assert_eq!(get_elves(), super::get_elves());
    }

    #[test]
    fn top1() {
        assert_eq!(get_elves().into_iter().max().unwrap(), super::get_max_elf());
    }

    #[test]
    fn top3() {
        let mut elves = get_elves();
        elves.sort_unstable();
        assert_eq!(
            elves.into_iter().rev().take(3).collect::<Vec<_>>(),
            super::get_top3_elfs()
        );
    }
}
