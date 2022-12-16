use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::iter;
use std::thread::current;

#[derive(Debug)]
struct Valve {
    id: String,
    rate: i32,
    next: Vec<String>,
}

#[derive(Debug)]
struct Solution {
    current_minutes: i32,
    current_id: String,
    pressure: i32,
    pressure_to_release: i32,
    open_valves: HashSet<String>,
    elephant: Option<String>,
    traces: String,
}

impl Solution {

    fn must_debug(&self) -> bool {
        let test = "-MvElDD-MvII-OpElDD-MvJJ-MvEl";
        self.traces.starts_with(test)
    }

    fn key(&self) -> String {
        let empty = "".to_string();
        /*let mut open_valves = self.open_valves.iter()
            .map(|op| op.to_string())
            .collect::<Vec<_>>();
        open_valves.sort();
        let open_valves = open_valves.join("-");
        format!("{}-{}-{}", self.current_id, self.elephant.as_ref().unwrap_or(&empty), open_valves)*/
        format!("{}-{}", self.current_id, self.elephant.as_ref().unwrap_or(&empty))

    }
    fn potentiel_max(&self, valves: &HashMap<String, Valve>) -> i32 {
        let mut potentiel_valves = valves.iter()
            .filter(|(_, valve)| !self.open_valves.contains(&valve.id))
            .map(|(_, valve)| valve.rate)
            .collect::<Vec<_>>();
        potentiel_valves.sort();
        potentiel_valves.reverse();
        let mut potential = self.pressure;

        let mut potentiel_valves = potentiel_valves.into_iter();
        let range: Vec<_> = if self.elephant.is_none() {
            (1..(30 - self.current_minutes)).rev().collect()
        } else {
            (1..(30 - self.current_minutes)).rev().flat_map(|i| vec![i, i]).collect()
        };

        for min in range.into_iter() {
            if let Some(potentiel_valve) = potentiel_valves.next() {
                potential += potentiel_valve * min
            } else {
                return potential
            }
        }

        potential

    }
    fn next(self, valves: &HashMap<String, Valve>) -> Vec<Solution> {
        let next_pressure = self.pressure + self.pressure_to_release;
        if self.open_valves.len() == valves.len() {
            return vec![self]
        }

        let open_valves_with_el: HashSet<_> = self.elephant.as_ref().map(|el|
            self.open_valves.iter()
                .map(|id| id.to_string())
                .chain(std::iter::once(el.to_string())
                )
                .collect()
        ).unwrap_or_default();


        let mut next_solutions = vec![];
        for (next_el, pressure, el_trace) in self.elephant
            .map(|elephant| {
                let mut next_elephant = vec![];
                if !self.open_valves.contains(&elephant) {
                    next_elephant.push((
                        Some(elephant.clone()),
                        valves.get(&elephant).as_ref().unwrap().rate,
                        format!("OpEl{}", elephant)
                    ));
                }
                for next in valves.get(&elephant).unwrap().next.iter() {
                    next_elephant.push((
                        Some(next.clone()),
                        0,
                        format!("MvEl{}", next)
                    ))
                }
                next_elephant
            }).unwrap_or(vec![(None, 0, "".to_string())]) {

            let next_open_values = if pressure > 0 {
                &open_valves_with_el
            } else {
                &self.open_valves
            };

            if !next_open_values.contains(&self.current_id) {
                next_solutions.push(Solution {
                    current_minutes: self.current_minutes + 1,
                    current_id: self.current_id.clone(),
                    pressure: next_pressure,
                    pressure_to_release: self.pressure_to_release + valves.get(&self.current_id).as_ref().unwrap().rate + pressure,
                    open_valves: next_open_values.iter()
                        .chain(iter::once(&self.current_id))
                        .map(|id| id.clone())
                        .collect(),
                    elephant: next_el.clone(),
                    traces: format!("{}-{}-Op{}", self.traces, el_trace, self.current_id),
                })
            }

            for possible_next in valves.get(&self.current_id).unwrap().next.iter() {
                next_solutions.push(Solution {
                    current_minutes: self.current_minutes + 1,
                    current_id: possible_next.clone(),
                    pressure: next_pressure,
                    pressure_to_release: self.pressure_to_release + pressure,
                    open_valves: next_open_values.clone(),
                    elephant: next_el.clone(),
                    traces:
                    format!("{}-{}-Mv{}", self.traces, el_trace, possible_next.clone()),
                })
            }
        }



        return next_solutions;
    }
}

pub fn day16() {
    let valves = include_str!("day16_sample.txt").lines()
        .map(|line| {
            let line = line.replace("Valve ", "")
                .replace(" has flow rate=", ";")
                .replace(" tunnels lead to valves ", "")
                .replace(" tunnel leads to valve ", "");
            let mut words = line
                .split(";");

            let id = words.next().unwrap().to_string();
            let rate = words.next().unwrap().parse().unwrap();
            let next = words.next().unwrap();

            (id.clone(), Valve {
                id,
                rate,
                next: next.split(", ")
                    .map(|word| word.to_string())
                    .collect(),
            })
        }).collect::<HashMap<_, _>>();


    /*let mut current_solutions = vec![Solution {
        current_minutes: 0,
        current_id: "AA".to_string(),
        pressure: 0,
        open_valves: HashSet::new(),
        elephant: None,
        traces: "".to_string(),
    }];

    for i in 1..30 {
        println!("i : {} ({})", i, current_solutions.len());
        let mut next: HashMap<String, Solution> = HashMap::new();
        let mut max_current_pressure = 0;
        for solution in current_solutions.into_iter()
            .flat_map(|solution| solution.next(&valves)) {
            let key = solution.key();
            let last_max = next.get(&key);
            if solution.pressure >= last_max.map(|sol| sol.pressure).unwrap_or(0) {
                max_current_pressure = max(max_current_pressure, solution.pressure);
                next.insert(key, solution);
            }

        }

        current_solutions =
            next.into_iter()
                .map(|(key, val)| val)
                .filter(|solution| solution.potentiel_max(&valves) > max_current_pressure || max_current_pressure <= solution.pressure)
                .collect()
    }

    let max_without_elephant = current_solutions.into_iter()
        .max_by_key(|sol| sol.pressure)
        .unwrap();


    println!("Step 1 : {:?}", max_without_elephant.pressure);*/

    let mut current_solutions = vec![Solution {
        current_minutes: 4,
        current_id: "AA".to_string(),
        pressure: 0,
        open_valves: HashSet::new(),
        elephant: Some("AA".to_string()),
        traces: "".to_string(),
        pressure_to_release: 0,
    }];

    for i in 1..26 {
        println!("i : {} ({})", i, current_solutions.len());
        let mut next: HashMap<String, Solution> = HashMap::new();
        let mut max_current_pressure = 0;
        for solution in current_solutions.into_iter()
            .flat_map(|solution| solution.next(&valves)) {

            /*if solution.must_debug() {
                println!("{:?}", solution);

            }*/
            let key = solution.key();
            let last_max = next.get(&key);
            if solution.pressure >= last_max.map(|sol| sol.pressure).unwrap_or(0) {
                max_current_pressure = max(max_current_pressure, solution.pressure);
                next.insert(key, solution);
            }

        }

        current_solutions = vec![];

        for (_, val) in next.into_iter() {
            current_solutions.push(val);
            /*if val.must_debug() {
                println!("{:?}", val.potentiel_max(&valves));
            }*/
            if val.potentiel_max(&valves) > max_current_pressure  || max_current_pressure <= val.pressure {
            }
        }
    }



    let max_with_elephant = current_solutions.into_iter()
        .max_by_key(|sol| sol.pressure + sol.pressure_to_release)
        .unwrap();


    println!("Step 1 : {:?}", max_with_elephant.pressure + max_with_elephant.pressure_to_release);

}

