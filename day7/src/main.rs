const INPUT: &[u8] = include_bytes!("../input");
const NUM_FOLDERS: usize = get_num_folders();
const FOLDERS: [(Option<usize>, u64); NUM_FOLDERS] = get_folders();
const FIRST: u64 = first();
const SECOND: u64 = second();

const fn get_num_folders() -> usize {
    let mut i = 0;
    let mut o = 0;
    while i < INPUT.len() {
        if INPUT[i] == b'$'
            && INPUT[i + 1] == b' '
            && INPUT[i + 2] == b'c'
            && INPUT[i + 3] == b'd'
            && INPUT[i + 4] == b' '
            && INPUT[i + 5] != b'.'
        {
            o += 1;
        }
        while INPUT[i] != 10 {
            i += 1;
        }
        i += 1;
    }
    o
}

const fn get_folders() -> [(Option<usize>, u64); NUM_FOLDERS] {
    let mut i = 0;
    let mut folders = [(None, 0); NUM_FOLDERS];
    let mut folder_index: i32 = -1;
    let mut current: Option<usize> = None;
    while i < INPUT.len() {
        match INPUT[i] {
            b'$' => match INPUT[i + 2] {
                b'c' => match INPUT[i + 5] {
                    b'.' => {
                        if let Some(c) = current {
                            current = folders[c].0;
                        } else {
                            unreachable!();
                        }
                    }
                    _ => {
                        folder_index += 1;
                        folders[folder_index as usize].0 = current;
                        current = Some(folder_index as usize);
                    }
                },
                b'l' => {}
                _ => unreachable!(),
            },
            b'd' => {}
            n => {
                let mut temp = [None; 10];
                temp[0] = Some(n);
                let mut j = 1;
                i += 1;
                while INPUT[i] != b' ' {
                    temp[j] = Some(INPUT[i]);
                    j += 1;
                    i += 1;
                }
                let size = get_num(temp);

                let mut parent = current;
                while let Some(id) = parent {
                    folders[id].1 += size;
                    parent = folders[id].0;
                }
            }
        }
        //next line
        while INPUT[i] != 10 {
            i += 1;
        }
        i += 1;
    }
    folders
}

const fn get_num(buf: [Option<u8>; 10]) -> u64 {
    let mut o = 0;
    let mut i = 9;
    let mut power = 0;
    loop {
        if let Some(n) = buf[i] {
            o += (n - b'0') as u64 * 10_u64.pow(power);
            power += 1;
        }
        if i == 0 {
            break;
        } else {
            i -= 1;
        }
    }
    o
}

const fn first() -> u64 {
    let mut i = 1; // skip root
    let mut sum = 0;
    while i < NUM_FOLDERS {
        if FOLDERS[i].1 < 100000 {
            sum += FOLDERS[i].1;
        }
        i += 1;
    }
    sum
}

const fn second() -> u64 {
    let free_space = 70000000 - FOLDERS[0].1;
    let space_to_free = 30000000 - free_space;
    let mut min_folder = 70000000;
    let mut i = 0;
    while i < NUM_FOLDERS {
        if FOLDERS[i].1 >= space_to_free && min_folder > FOLDERS[i].1 {
            min_folder = FOLDERS[i].1;
        }
        i += 1;
    }
    min_folder
}

fn main() {
    println!("first {FIRST}");
    println!("second {SECOND}");
}

#[cfg(test)]
mod tests {
    enum Line<'a> {
        Cd(&'a str),
        CdDotDot,
        Ls,
        Dir(&'a str),
        File(u64, &'a str),
    }

    struct Folder {
        parent: Option<usize>,
        files: u64,
    }

    fn get_folders() -> Vec<Folder> {
        let input = include_str!("../input");
        let mut folders: Vec<Folder> = vec![];
        let mut current = None;
        input
            .lines()
            .map(|s| {
                let mut temp = s.split(' ');
                match temp.next().unwrap() {
                    "$" => match temp.next().unwrap() {
                        "cd" => match temp.next().unwrap() {
                            ".." => Line::CdDotDot,
                            s => Line::Cd(s),
                        },
                        "ls" => Line::Ls,
                        _ => unreachable!(),
                    },
                    "dir" => Line::Dir(temp.next().unwrap()),
                    n => Line::File(n.parse().unwrap(), temp.next().unwrap()),
                }
            })
            .for_each(|l| match l {
                Line::Cd(_name) => {
                    folders.push(Folder {
                        parent: current,
                        files: 0,
                    });
                    current = Some(folders.len() - 1);
                }
                Line::CdDotDot => {
                    current = folders[current.unwrap()].parent;
                }
                Line::Ls => {}
                Line::Dir(_name) => {}
                Line::File(size, _name) => {
                    let mut parent = current;
                    while let Some(id) = parent {
                        folders[id].files += size;
                        parent = folders[id].parent;
                    }
                }
            });
        folders
    }

    #[test]
    fn first() {
        let sum = get_folders()
            .iter()
            .skip(1) // skip root
            .filter_map(|f| (f.files < 100000).then_some(f.files))
            .sum::<u64>();
        assert_eq!(sum, super::FIRST); // 1477771
    }
    #[test]
    fn second() {
        let folders = get_folders();
        let free_space = 70000000 - folders[0].files;
        let space_to_free = 30000000 - free_space;
        let mut temp = folders
            .iter()
            .filter_map(|f| (f.files >= space_to_free).then_some(f.files))
            .collect::<Vec<u64>>();
        temp.sort_unstable();
        assert_eq!(temp[0], super::SECOND); // 3579501
    }
}
