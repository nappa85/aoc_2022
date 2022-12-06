const INPUT: &[u8] = include_bytes!("../input");
const FIRST: usize = first_unique(4);
const SECOND: usize = first_unique(14);

const fn is_unique(start: usize, size: usize) -> bool {
    let mut i = start;
    while i < start + size - 1 {
        let mut j = i + 1;
        while j < start + size {
            if INPUT[i] == INPUT[j] {
                return false;
            }
            j += 1;
        }
        i += 1;
    }
    true
}

const fn first_unique(size: usize) -> usize {
    let mut i = 0;
    while i < INPUT.len() - size {
        if is_unique(i, size) {
            return i + size;
        }
        i += 1;
    }
    unreachable!()
}

fn main() {
    println!("first {FIRST}");
    println!("second {SECOND}");
}

#[cfg(test)]
mod tests {
    fn is_unique(bytes: &[u8]) -> bool {
        for i in 0..bytes.len() {
            for j in (i + 1)..bytes.len() {
                if bytes[i] == bytes[j] {
                    return false;
                }
            }
        }
        true
    }
    #[test]
    fn first() {
        let pos = super::INPUT.windows(4).position(is_unique);
        assert_eq!(super::FIRST, pos.unwrap_or_default() + 4); //1757
    }
    #[test]
    fn second() {
        let pos = super::INPUT.windows(14).position(is_unique);
        assert_eq!(super::SECOND, pos.unwrap_or_default() + 14); //2950
    }
}
