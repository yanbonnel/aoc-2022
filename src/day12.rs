use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::thread::current;

#[derive(Debug)]
struct Solution {
    already_visited: Vec<(usize, usize)>,
    current_point: (usize, usize)
}

fn get_high(grid: &Vec<Vec<char>>, x: usize, y: usize) -> char {
    let car = grid[y][x];
    match car {
        'E' => 'z',
        'S' => 'a',
        _ => car
    }
}

impl Solution {

    pub fn is_final(&self, grid: &Vec<Vec<char>>) -> bool {
        let (x, y) = self.current_point;
        grid[y][x] == 'E'
    }

    pub fn next(&self, grid: &Vec<Vec<char>>) -> Vec<Solution> {
        let current_high = get_high(grid, self.current_point.0, self.current_point.1) as u32;
        let height = grid.len() as i32;
        let width = grid.first().unwrap().len() as i32;

        let current_x = self.current_point.0 as i32;
        let current_y = self.current_point.1 as i32;

        vec![
            (current_x, current_y + 1),
            (current_x, current_y - 1),
            (current_x + 1, current_y),
            (current_x - 1, current_y),
        ].into_iter()
            .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < width && *y < height)
            .map(|(x, y)| (x as usize, y as usize))
            .filter(|point| !self.already_visited.contains(point))
            .filter(|(x, y)| {
                (get_high(grid, *x, *y) as u32).abs_diff(current_high) <= 1
                || (get_high(grid, *x, *y) as u32) < current_high
            })
            .map(|(x, y)| {
                let mut already_visited = self.already_visited.clone();
                already_visited.push((x, y));
                Solution {
                    already_visited,
                    current_point: (x, y)
                }
            }).collect()
    }

}

pub fn day12() {
    let grid = include_str!("day12.txt").lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let (start_x, start_y, _) = grid.iter().enumerate().flat_map(|(y, line)| line.iter().enumerate().map(move |(x, car)| (x, y, *car)))
        .find(|(_, _, car)| *car == 'S').unwrap();



    let step_1_result = find_result(&grid, start_x, start_y);
    println!("Step 1 result : {}", step_1_result);


    let step2 = grid.iter().enumerate().flat_map(|(y, line)| line.iter().enumerate().map(move |(x, car)| (x, y, *car)))
        .filter(|(_, _, car)| *car == 'S' || *car == 'a')
        .map(|(x, y, _)| find_result(&grid, x, y))
        .min().unwrap();


    println!("Step 2 result : {}", step2);




}

fn find_result(grid: &Vec<Vec<char>>, start_x: usize, start_y: usize) -> usize {

    let mut solutions: Vec<Solution> = vec![
        Solution {
            already_visited: vec![(start_x, start_y)],
            current_point: (start_x, start_y)
        }
    ];
    let mut better: HashMap<(usize, usize), usize> = HashMap::new();
    better.insert((start_x, start_y), 1);

    loop {
        if solutions.len() == 0 {
            return 9999999
        }
        let next = solutions.remove(0);

        if next.is_final(&grid) {
            return next.already_visited.len() - 1;
        }
        for next_solution in next.next(&grid) {
            if better.get(&next_solution.current_point)
                .map(|v| *v)
                .unwrap_or(9999999) > next_solution.already_visited.len() {
                better.insert(next_solution.current_point, next_solution.already_visited.len());
                solutions.push(next_solution);
            }
        }
    }
}