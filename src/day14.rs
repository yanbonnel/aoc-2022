use std::cmp::{max, min};
use std::collections::HashSet;

fn str_to_point(line: &str) -> Point {
    let mut words = line.split(",");
    (words.next().unwrap().parse().unwrap(), words.next().unwrap().parse().unwrap())
}

type Point = (u16, u16);

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn from_str(line: &[String]) -> Self {
        Line {
            start: str_to_point(&line[0]),
            end: str_to_point(&line[1]),
        }
    }

    fn points(&self) -> HashSet<Point> {
        (min(self.start.0, self.end.0)..=max(self.start.0, self.end.0))
            .flat_map(|x|
                (min(self.start.1, self.end.1)..=max(self.start.1, self.end.1)).map(move |y|
                    (x, y)
                )
            ).collect()
    }
}

fn print_grid(grid: &HashSet<Point>) {
    let min_y = grid.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = grid.iter().map(|(_, y)| *y).max().unwrap();
    let min_x = grid.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = grid.iter().map(|(x, _)| *x).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if grid.contains(&(x, y)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
}

fn next(point: Point, grid: &HashSet<Point>, max_y: u16) -> Option<Point> {
    let next_pt = drop_until_block(point, grid, max_y);
    if let Some(next_pt) = next_pt {
        // try left
        let left = (next_pt.0 - 1, next_pt.1 + 1);
        if !grid.contains(&left) {
            return next(left, grid, max_y);
        }
        // try right
        let right = (next_pt.0 + 1, next_pt.1 + 1);
        if !grid.contains(&right) {
            return next(right, grid, max_y);
        }
        return Some(next_pt);
    } else {
        None
    }
}

fn next_2(point: Point, grid: &HashSet<Point>, max_y: u16) -> Point {
    let next_pt = drop_until_block_2(point, grid, max_y);
    // try left
    if next_pt.1 >= max_y + 1 {
        return next_pt
    }
    let left = (next_pt.0 - 1, next_pt.1 + 1);
    if !grid.contains(&left) {
        return next_2(left, grid, max_y);
    }

    // try right
    let right = (next_pt.0 + 1, next_pt.1 + 1);
    if !grid.contains(&right) {
        return next_2(right, grid, max_y);
    }
    return next_pt;
}

fn drop_until_block(mut point: Point, grid: &HashSet<Point>, max_y: u16) -> Option<Point> {
    while !grid.contains(&point) {
        if point.1 > max_y {
            return None;
        }
        point = (point.0, point.1 + 1);
    }
    point = (point.0, point.1 - 1);
    if grid.contains(&point) {
        None
    } else {
        Some(point)
    }
}

fn drop_until_block_2(mut point: Point, grid: &HashSet<Point>, max_y: u16) -> Point {
    while !grid.contains(&point) && point.1 < max_y + 2 {
        point = (point.0, point.1 + 1);
    }
    point = (point.0, point.1 - 1);
    point
}

pub fn day14() {
    let elements = include_str!("day14.txt").lines()
        .flat_map(|line| {
            let points = line.split(" -> ")
                .map(|line| line.to_string())
                .collect::<Vec<_>>();
            points
                .windows(2)
                .map(|points|
                    Line::from_str(points)
                ).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();


    let mut grid = elements
        .iter().flat_map(|line| line.points())
        .collect::<HashSet<_>>();

    let mut counter = 0;

    let max_y = grid.iter().map(|(_, y)| *y).max().unwrap();

    while let Some(next_pt) = next((500, 0), &grid, max_y) {
        counter += 1;
        grid.insert(next_pt);
        //print_grid(&grid);
    }


    let mut counter_2 = 0;
    let mut grid2 = elements
        .iter().flat_map(|line| line.points())
        .collect::<HashSet<_>>();

    let mut next_pt = next_2((500, 0), &grid2, max_y);
    while next_pt != (500, 0) {
        counter_2 += 1;
        grid2.insert(next_pt);
        next_pt = next_2((500, 0), &grid2, max_y);

    }
    counter_2 += 1;
    grid2.insert(next_pt);


    println!("Step 2 : {}", counter_2);
}