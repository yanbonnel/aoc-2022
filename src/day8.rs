use std::cmp::{max, min};

pub fn get_tree_size(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    grid[y][x]
}

pub fn day8() {
    let grid: Vec<Vec<_>> = include_str!("./day8.txt")
        .split("\n").map(|line| line.chars()
        .map(|car| car.to_digit(10).unwrap()).collect()
    ).collect();

    let width = grid.first().unwrap().len();
    let height = grid.len();

    let nb_visible = grid.iter().enumerate().flat_map(|(y, row)| {
        let grid = grid.clone();
        row.iter().enumerate().filter(move |(x, tree_size)| {
            let tree_size = **tree_size;
            let x = *x;
            if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                return true;
            }
            // has bigger tree on left
            let is_visible_left = (0..x).all(|other_x|
                get_tree_size(&grid, other_x, y) < tree_size
            );
            let is_visible_top = (0..y).all(|other_y|
                get_tree_size(&grid, x, other_y) < tree_size
            );
            let is_visible_right = ((x + 1)..width).all(|other_x|
                get_tree_size(&grid, other_x, y) < tree_size
            );
            let is_visible_bottom = ((y + 1)..height).all(|other_y|
                get_tree_size(&grid, x, other_y) < tree_size
            );
            is_visible_left || is_visible_top || is_visible_right || is_visible_bottom
        })
    }).count();

    println!("Step 1 : {}", nb_visible);

    let max_scenic = grid.iter().enumerate().flat_map(|(y, row)|
        row.iter().enumerate().map(move |(x, tree_size)|
            (x, y, *tree_size)
        )
    ).filter(|(x, y, tree_size)|
        !(*x == 0 || *y == 0 || *x == width - 1 || *y == height - 1)
    ).map(|(x, y, tree_size)| {
        // left
        let tree_visible_left = min((0..x).rev().take_while(|other_x|
            get_tree_size(&grid, *other_x, y) < tree_size
        ).count() + 1, x);
        let tree_visible_top = min((0..y).rev().take_while(|other_y|
            get_tree_size(&grid, x, *other_y) < tree_size
        ).count() + 1, y);

        let tree_visible_right = min(((x + 1)..width).take_while(|other_x|{
            get_tree_size(&grid, *other_x, y) < tree_size
    }).count() + 1, width - x - 1);

        let tree_visible_bottom = min(((y + 1)..height).take_while(|other_y|
            get_tree_size(&grid, x, *other_y) < tree_size
        ).count() + 1, height - y - 1);

        let result = tree_visible_left * tree_visible_top * tree_visible_right * tree_visible_bottom;
        result
    }).max().unwrap();

    println!("Step 2 : {}", max_scenic);
}