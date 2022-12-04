fn main() {
    let input = include_str!("../input");
    let tot = input
        .lines()
        .map(|s| {
            let mut iter = s.split(',').map(|r| {
                let mut iter = r.split('-').map(|i| i.parse::<u16>().unwrap());
                (iter.next().unwrap())..=(iter.next().unwrap())
            });
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .filter(|(a, b)| {
            (a.contains(b.start()) && a.contains(b.end()))
                || (b.contains(a.start()) && b.contains(a.end()))
        })
        .count();
    println!("contained {tot}"); // 547
    let tot = input
        .lines()
        .map(|s| {
            let mut iter = s.split(',').map(|r| {
                let mut iter = r.split('-').map(|i| i.parse::<u16>().unwrap());
                (iter.next().unwrap())..=(iter.next().unwrap())
            });
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .filter(|(a, b)| {
            a.contains(b.start())
                || a.contains(b.end())
                || b.contains(a.start())
                || b.contains(a.end())
        })
        .count();
    println!("overlapping {tot}"); // 547
}
