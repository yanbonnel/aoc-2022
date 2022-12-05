

pub fn day1() {
    let mut elves = vec![0u32];
    for line in include_str!("./day1.txt")
        .split("\n") {
        if line.is_empty() {
            elves.push(0)
        } else {
            let cals: u32 = line.parse().unwrap();
            let last_elf = elves.pop().unwrap();
            elves.push(
                last_elf + cals
            );
        }
    }

    elves.sort();
    elves.reverse();





    let max = elves.iter().max().unwrap();
    println!("step1 {}", max);

    let step2: u32 = elves.iter().take(3).sum();
    println!("step2 {}", step2);

}
