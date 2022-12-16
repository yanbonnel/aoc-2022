use std::cmp::max;
use std::collections::HashSet;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

type Point = (i32, i32);

pub fn line_to_point(line: &str) -> Point {
    let mut words = line.split(", ");
    let x = words.next().unwrap().replace("x=", "").parse().unwrap();
    let y = words.next().unwrap().replace("y=", "").parse().unwrap();
    (x, y)
}

pub fn distance(p1: &Point, p2: &Point) -> u32 {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

struct Radar {
    sensor: Point,
    distance_b: u32,
}

pub fn day15() {
    let y_search: i32 = 2000000;
    //let y_search: i32 = 10;
    let elements = include_str!("day15.txt").lines()
        .map(|line| {
            let line = line.replace("Sensor at ", "")
                .replace(" closest beacon is at ", "");
            let mut words = line
                .split(":");
            let sensor = line_to_point(words.next().unwrap());
            let beacon = line_to_point(words.next().unwrap());
            (sensor, beacon)
        })
        .collect::<Vec<_>>();

    let beacon_on_y: HashSet<_> = elements.iter()
        .filter(|(_, b)| b.1 == y_search)
        .map(|(_, (b_x, _))| *b_x)
        .collect();

    let radars = elements.iter()
        .map(|(s, b)|
            Radar {
                sensor: s.clone(),
                distance_b: distance(s, b)
            }
        ).collect::<Vec<_>>();

    let x_blocked = x_blocked_at_y(y_search, &radars);

    let mut x_blocked = x_blocked.into_iter().collect::<Vec<_>>();
    x_blocked.sort();

    println!("step 1 ={}", x_blocked.len() - beacon_on_y.len());

    //let max_x_y: i32 = 20;
    let max_x_y: i32 = 4000000;

    let start = SystemTime::now();

    for y in 0..=max_x_y {
        if y % 100000 == 0 {
            let since_the_epoch = SystemTime::now()
                .duration_since(start).unwrap();

            println!("{} - {}/{}", since_the_epoch.as_millis(), y, max_x_y);
        }
        let mut ranges = ranges_at_y(y, &radars).into_iter();
        let mut current = vec![Range {
            min: 0,
            max: max_x_y,
        }];

        while let Some(other) = ranges.next() {
            if current.is_empty() {
                break;
            }
            current = current.into_iter()
                .flat_map(|range|
                    range.minus(&other)
                ).collect();
        }
        if !current.is_empty() {
            let result = current.first().unwrap();
            if result.min != result.max {
                panic!()
            }

            let step2_result = (result.min as i64) * 4000000 + (y as i64);

            println!("Step 2 = {:?}", step2_result);
            break;
        }
    }



    /*let static_check = radars.iter().map(
        |radar| radar.to_static_test()
    ).collect::<Vec<_>>().join(" && ");

    println!("{}", static_check);*/

    /*let result = step2_loop(max_x_y);

    let step2_result = (result.0 as i64) * 4000000 + (result.1 as i64);

    println!("Step 2 = {:?}", step2_result)
**/


}

struct Range {
    min: i32,
    max: i32,
}

impl Range {
    fn minus(self, other: &Range) -> Vec<Range> {

        if self.min < other.min && self.max > other.max {
            vec![
                Range {
                    min: self.min,
                    max: other.min - 1
                },
                Range {
                    min: other.max + 1,
                    max: self.max
                }
            ]
        } else if self.min > other.max || self.max < other.min {
            vec![self]
        } else if self.min >= other.min && self.max > other.max {
            vec![
                Range {
                    min: other.max + 1,
                    max: self.max
                }
            ]
        } else if self.min < other.min && self.max <= other.max {
            vec![
                Range {
                    min: self.min,
                    max: other.min - 1
                }
            ]
        } else if self.min >= other.min && self.max <= other.max {
            vec![]
        } else {
            vec![self]
        }
    }

    fn all_x(&self) -> Vec<i32> {
        (self.min..=self.max).collect()
    }
}

fn ranges_at_y(y_search: i32, radars: &Vec<Radar>) -> Vec<Range> {
    radars.iter().flat_map(
        |radar| {
            let d = radar.distance_b;
            let dist_y = y_search.abs_diff(radar.sensor.1);
            let diff_dist = d.abs_diff(dist_y) as i32;
            if dist_y <= d {
                Some(Range {
                    min: radar.sensor.0 - diff_dist,
                    max: radar.sensor.0 + diff_dist
                })
            } else {
                None
            }
        }
    ).collect()
}


fn x_blocked_at_y(y_search: i32, radars: &Vec<Radar>) -> HashSet<i32> {
    ranges_at_y(y_search, radars)
        .into_iter()
        .flat_map(|range| range.all_x())
        .collect()
}