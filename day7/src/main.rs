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

fn main() {
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
                        _ => unreachable!(),
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
    let sum = folders
        .iter()
        .skip(1) // skip root
        .filter_map(|f| (f.files < 100000).then_some(f.files))
        .sum::<u64>();
    println!("{sum}"); //1477771
    let free_space = 70000000 - folders[0].files;
    let space_to_free = 30000000 - free_space;
    let mut temp = folders
        .iter()
        .filter_map(|f| (f.files >= space_to_free).then_some(f.files))
        .collect::<Vec<u64>>();
    temp.sort_unstable();
    println!("{}", temp[0]); //3579501
}
