fn main() {
    let input = include_bytes!("../input");
    let pos = input.windows(4).position(first_unique);
    println!("{}", pos.unwrap_or_default() + 4);
    let pos = input.windows(14).position(first_unique);
    println!("{}", pos.unwrap_or_default() + 14);
}

fn first_unique(bytes: &[u8]) -> bool {
    for i in 0..bytes.len() {
        for j in (i + 1)..bytes.len() {
            if bytes[i] == bytes[j] {
                return false;
            }
        }
    }
    true
}
