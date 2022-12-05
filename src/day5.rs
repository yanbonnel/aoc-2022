use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn day5() {

    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
    let mut stacks_step2: HashMap<usize, Vec<char>> = HashMap::new();

    let mut stack_finished = false;

    for line in include_str!("./day5.txt")
        .split("\n") {

        if line.is_empty() {
            stack_finished = true;
            stacks_step2 = stacks.clone();
            continue
        }

        if !stack_finished {
            for (index, cars) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
                if cars[0] != ' ' {
                    let car = cars[1];
                    match stacks.entry(index + 1) {
                        Entry::Occupied(o) => o.into_mut(),
                        Entry::Vacant(v) => v.insert(vec![]),
                    }.insert(0, car);
                }
            }
        } else {
            let mut splited = line.split(" ").collect::<Vec<_>>();
            let nb_move: usize = splited[1].parse().unwrap();
            let from: usize = splited[3].parse().unwrap();
            let to: usize = splited[5].parse().unwrap();

            let mut stack_from_step2 = vec![];

            for _ in 0..nb_move {
                let car_from = stacks.get_mut(&from).unwrap().pop().unwrap();
                stacks.get_mut(&to).unwrap().push(car_from);

                let car_from = stacks_step2.get_mut(&from).unwrap().pop().unwrap();
                stack_from_step2.insert(0, car_from);
            }

            stacks_step2.get_mut(&to).unwrap().append(&mut stack_from_step2);


        }
    }

    let result = (1..=stacks.len()).flat_map(|index|
        stacks.get(&index).and_then(|stack| stack.last().map(|car| *car))
    ).collect::<String>();

    println!("Result step1 = {:?}", result);


    let result_step2 = (1..=stacks_step2.len()).flat_map(|index|
        stacks_step2.get(&index).and_then(|stack| stack.last().map(|car| *car))
    ).collect::<String>();

    println!("Result step2 = {:?}", result_step2);
}