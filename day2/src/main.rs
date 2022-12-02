const INPUT: &[u8] = include_bytes!("../input");
const SUM_AS_MINE: u16 = get_sum_as_mine();
const SUM_AS_OUTCOME: u16 = get_sum_as_outcome();

const fn get_sum_as_mine() -> u16 {
    let mut index = 0;
    let mut sum = 0;
    while index < INPUT.len() {
        sum += Elf::new(INPUT[index]).add_mine(Mine::new(INPUT[index + 2])) as u16;
        index += 4;
    }
    sum
}

const fn get_sum_as_outcome() -> u16 {
    let mut index = 0;
    let mut sum = 0;
    while index < INPUT.len() {
        sum += Elf::new(INPUT[index]).add_outcome(Outcome::new(INPUT[index + 2])) as u16;
        index += 4;
    }
    sum
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
enum Elf {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl Elf {
    const fn new(b: u8) -> Self {
        match b {
            b'A' => Elf::Rock,
            b'B' => Elf::Paper,
            b'C' => Elf::Scissor,
            _ => unreachable!(),
        }
    }

    const fn add_mine(self, rhs: Mine) -> u8 {
        rhs as u8
            + match (self, rhs) {
                (Elf::Rock, Mine::Rock)
                | (Elf::Paper, Mine::Paper)
                | (Elf::Scissor, Mine::Scissor) => 3, //draw
                (Elf::Rock, Mine::Paper)
                | (Elf::Paper, Mine::Scissor)
                | (Elf::Scissor, Mine::Rock) => 6, //win
                (Elf::Paper, Mine::Rock)
                | (Elf::Scissor, Mine::Paper)
                | (Elf::Rock, Mine::Scissor) => 0, //lose
            }
    }

    const fn add_outcome(self, rhs: Outcome) -> u8 {
        rhs as u8
            + match (self, rhs) {
                (Elf::Rock, Outcome::Draw)
                | (Elf::Paper, Outcome::Lose)
                | (Elf::Scissor, Outcome::Win) => Mine::Rock as u8,
                (Elf::Rock, Outcome::Win)
                | (Elf::Paper, Outcome::Draw)
                | (Elf::Scissor, Outcome::Lose) => Mine::Paper as u8,
                (Elf::Paper, Outcome::Win)
                | (Elf::Scissor, Outcome::Draw)
                | (Elf::Rock, Outcome::Lose) => Mine::Scissor as u8,
            }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
enum Mine {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl Mine {
    const fn new(b: u8) -> Self {
        match b {
            b'X' => Mine::Rock,
            b'Y' => Mine::Paper,
            b'Z' => Mine::Scissor,
            _ => unreachable!(),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl Outcome {
    const fn new(b: u8) -> Self {
        match b {
            b'X' => Outcome::Lose,
            b'Y' => Outcome::Draw,
            b'Z' => Outcome::Win,
            _ => unreachable!(),
        }
    }
}

fn main() {
    println!("Sum as mine: {SUM_AS_MINE}");
    println!("Sum as outcome: {SUM_AS_OUTCOME}");
}

#[cfg(test)]
mod tests {
    use std::ops::Add;

    impl From<u8> for super::Elf {
        fn from(b: u8) -> Self {
            super::Elf::new(b)
        }
    }
    impl From<u8> for super::Mine {
        fn from(b: u8) -> Self {
            super::Mine::new(b)
        }
    }
    impl From<u8> for super::Outcome {
        fn from(b: u8) -> Self {
            super::Outcome::new(b)
        }
    }
    impl Add<super::Mine> for super::Elf {
        type Output = u8;
        fn add(self, rhs: super::Mine) -> Self::Output {
            self.add_mine(rhs)
        }
    }
    impl Add<super::Outcome> for super::Elf {
        type Output = u8;
        fn add(self, rhs: super::Outcome) -> Self::Output {
            self.add_outcome(rhs)
        }
    }

    #[test]
    fn first() {
        let sum = include_bytes!("../input")
            .chunks(4)
            .map(|s| (super::Elf::from(s[0]) + super::Mine::from(s[2])) as u16)
            .sum::<u16>();
        assert_eq!(sum, super::SUM_AS_MINE); //13009
    }
    #[test]
    fn second() {
        let sum = include_bytes!("../input")
            .chunks(4)
            .map(|s| (super::Elf::from(s[0]) + super::Outcome::from(s[2])) as u16)
            .sum::<u16>();
        assert_eq!(sum, super::SUM_AS_OUTCOME); //10398
    }
}
