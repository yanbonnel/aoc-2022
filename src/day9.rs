use std::collections::HashSet;

pub fn next_position(dir: &str, (head_x, head_y): (i32, i32)) -> (i32, i32) {
    match dir {
        "R" => (head_x + 1, head_y),
        "U" => (head_x, head_y + 1),
        "L" => (head_x - 1, head_y),
        "D" => (head_x, head_y - 1),
        _ => panic!()
    }
}

fn next_tail((head_x, head_y): (i32, i32), (tail_x, tail_y): (i32, i32)) -> (i32, i32) {
    let x_steps = (head_x - tail_x);
    let y_steps = (head_y - tail_y);
    if x_steps.abs() <= 1 && y_steps.abs() <= 1 {
        return (tail_x, tail_y)
    }

    let next_x = if x_steps < 0 {
        tail_x - 1
    } else if x_steps > 0 {
        tail_x + 1
    } else {
        tail_x
    };
    let next_y = if y_steps < 0 {
        tail_y - 1
    } else if y_steps > 0 {
        tail_y + 1
    }else {
        tail_y
    };
    (next_x, next_y)
    
}

pub fn day9() {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut tail_position = (0, 0);
    let mut head_position = (0, 0);

    visited.insert((0, 0));

    for line in include_str!("./day9.txt")
        .split("\n") {
        let mut words = line.split(" ");
        let dir = words.next().unwrap();
        let nb: i32 = words.next().unwrap().parse().unwrap();

        for _ in 0..nb {
            head_position = next_position(dir, head_position);
            tail_position = next_tail(head_position, tail_position);
            visited.insert(tail_position);
        }
    }

    println!("Step 1 : {}", visited.len());

    let mut rope = vec![
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];

    visited.clear();

    for line in include_str!("./day9.txt")
        .split("\n") {
        let mut words = line.split(" ");
        let dir = words.next().unwrap();
        let nb: i32 = words.next().unwrap().parse().unwrap();

        for _ in 0..nb {
            let mut next_rope = vec![];

            next_rope.insert(0, next_position(dir, rope.pop().unwrap()));

            while let Some(next) = rope.pop() {
                next_rope.insert(0, next_tail(*next_rope.first().unwrap(), next))
            }

            visited.insert(*next_rope.first().unwrap());

            rope = next_rope.clone();
        }
    }

    println!("Step 2 : {}", visited.len());

}