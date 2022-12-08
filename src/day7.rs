
#[derive(Clone, Debug)]
enum DirOrFile {
    Dir(String, Vec<DirOrFile>),
    File(String, u32)
}

impl DirOrFile {
    fn cd(&mut self, dir: &Vec<String>) -> &mut DirOrFile {
        if dir.is_empty() {
            return self;
        }
        let first = dir.first().unwrap();
        let rest = dir.iter().skip(1)
            .map(|name| name.clone())
            .collect::<Vec<_>>();
        match self {
            DirOrFile::Dir(_, content) => {
                content.iter_mut().find(|dir_or_file|
                    dir_or_file.is_dir_named(&first)
                ).unwrap()
            }
            DirOrFile::File(_, _) => panic!("Not a dir")
        }.cd(&rest.into())
    }

    fn dir_sizes(&self) -> (Vec<(String, u32)>, u32) {
        match self {
            DirOrFile::Dir(name, content) => {
                let (mut dir_vec, dir_size) = content.iter().map(|content| content.dir_sizes()).fold((vec![], 0), |(mut acc_vec, acc_size), (item_vec, item_size)| {
                    acc_vec.extend(item_vec);
                    (acc_vec, acc_size + item_size)
                });
                dir_vec.push((name.to_string(), dir_size));
                (dir_vec, dir_size)

            },
            DirOrFile::File(_, size) => (vec![], *size)
        }
    }

    fn add_dir_or_file(&mut self, dir_or_file: DirOrFile) {
        match self {
            DirOrFile::Dir(_, content) => content.push(dir_or_file),
            DirOrFile::File(_, _) => panic!("Not a dir")
        }
    }

    fn is_dir_named(&self, name: &str) -> bool {
        match self {
            DirOrFile::Dir(dir_name, _) => dir_name == name,
            DirOrFile::File(_, _) => false
        }
    }
}


pub fn day7() {
    let mut current_dir: Vec<String> = vec![];
    let mut fs = DirOrFile::Dir("/".to_string(), vec![]);
    for line in include_str!("./day7.txt")
        .split("\n") {
        let mut words = line.split(" ");
        let first = words.next().unwrap();
        if first == "$" {
            let command = words.next().unwrap();
            if command == "cd" {
                let dir = words.next().unwrap();
                match dir {
                    "/" => {
                        current_dir.clear();
                    },
                    ".." => {
                        current_dir.pop();
                    },
                    _ => {
                        current_dir.push(
                            dir.to_string()
                        )
                    }
                }
            }
        } else if first == "dir" {
            let dir_name = words.next().unwrap();
            fs.cd(&current_dir).add_dir_or_file(DirOrFile::Dir(dir_name.to_string(), vec![]))
        } else {
            let size = first.parse().unwrap();
            let name = words.next().unwrap();
            fs.cd(&current_dir).add_dir_or_file(DirOrFile::File(name.to_string(), size));
        }
    }

    let step1_result: u32 = fs.dir_sizes().0.into_iter().filter(|(_, size)|
       *size < 100000u32
    ).map(|(_, size)| size).sum();

    println!("Step 1 = {}", step1_result);

    let (dirs, total_size) = fs.dir_sizes();

    let size_to_remove = 30000000 - (70000000 - total_size);

    let mut dirs_enough_big = dirs.into_iter().map(|(_, size)| size)
        .filter(|size| *size >= size_to_remove)
        .collect::<Vec<_>>();
    dirs_enough_big.sort();

    println!("Step 2 = {}", dirs_enough_big.first().unwrap());



}