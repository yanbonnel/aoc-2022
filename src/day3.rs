use std::collections::HashSet;

pub fn car_score(car: char) -> u8 {
    if car.is_lowercase() {
        return (car as u8) - ('a' as u8) + 1
    }
    return (car as u8) - ('A' as u8) + 27
}

pub fn day3() {

    let mut total_score : u32 = 0;

    for line in include_str!("./day3.txt")
        .split("\n") {
        let cars = line.chars().collect::<Vec<_>>();
        let mut chuncks = cars.chunks(line.len() / 2);

        let part1 = chuncks.next().unwrap().into_iter().collect::<HashSet<_>>();

        let part2 = chuncks.next().unwrap();

        let common = *part2.into_iter().filter(|car| part1.contains(car)).next().unwrap();
        let score = car_score(common);
        total_score = total_score + (score as u32);
    }

    println!("Step 1 : {}", total_score);

    let mut total_score_step2: u32 = 0;

    for groups in include_str!("./day3.txt")
        .split("\n").collect::<Vec<_>>().chunks(3) {

        let group1 = groups[0].chars().collect::<HashSet<_>>();

        let group2 = groups[1].chars().collect::<HashSet<_>>();
        let group3 = groups[2];

        let common = group3.chars().filter(|car| group1.contains(car) && group2.contains(car)).next().unwrap();

        let score = car_score(common);
        total_score_step2 = total_score_step2 + (score as u32);
    }

    println!("Step 2 : {}", total_score_step2);




}