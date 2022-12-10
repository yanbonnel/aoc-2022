
pub fn calculate_signal_stenghts_if_needed(current_cycle: i32, x_value: i32) -> i32 {
    if (current_cycle - 20)%40 == 0 {
        let result = current_cycle * x_value;
        result
    } else {
        0
    }
}

pub fn calculate_screen_symbol(screen: &mut Vec<Vec<char>>, x_value: i32) {
    let mut current_pos: i32 = screen.last().map(|last| last.len() as i32).unwrap_or(0);
    if current_pos >= 40 {
        current_pos = 0;
    }

    let car_to_add = if current_pos.abs_diff(x_value) <= 1 {
        '#'
    } else {
        '.'
    };
    if current_pos == 0 {
        screen.push(vec![car_to_add]);
    } else {
        screen.last_mut().unwrap().push(car_to_add);
    }
}

pub fn day10() {

    let mut x = 1;

    let mut current_cycle = 0;

    let mut signal_stenght = 0;

    let mut screen: Vec<Vec<char>> = vec![];

    for line in include_str!("day10.txt").lines() {
        let mut words = line.split(" ");
        match words.next().unwrap() {
            "noop" => {
                current_cycle += 1;
                signal_stenght += calculate_signal_stenghts_if_needed(current_cycle, x);
                calculate_screen_symbol(&mut screen, x);
            },
            "addx" => {
                let inc: i32 = words.next().unwrap().parse().unwrap();
                current_cycle+=1;
                signal_stenght += calculate_signal_stenghts_if_needed(current_cycle, x);
                calculate_screen_symbol(&mut screen, x);
                current_cycle+=1;
                signal_stenght += calculate_signal_stenghts_if_needed(current_cycle, x);
                calculate_screen_symbol(&mut screen, x);
                x += inc;
            },
            _ => panic!()
        }

    }

    println!("Step 1 : {}", signal_stenght);

    for line in screen {
        for car in line {
            print!("{}", car);
        }
        println!();
    }
}