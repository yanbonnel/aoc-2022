use std::cmp::Ordering;

#[derive(Debug)]
enum Element {
    Int(u16),
    List(Vec<Element>)
}

#[derive(PartialEq)]
enum Smaller {
    Left,
    Right,
    Equal
}

fn is_left_smaller(left: &Element, right: &Element) -> Smaller {
    match (left, right) {
        (Element::Int(left_int), Element::Int(right_int)) if left_int < right_int => Smaller::Left,
        (Element::Int(left_int), Element::Int(right_int)) if left_int > right_int => Smaller::Right,
        (Element::Int(_), Element::Int(_)) => Smaller::Equal,
        (Element::List(left_els), Element::List(right_els)) => {
            let mut left_iter = left_els.iter();
            let mut right_iter = right_els.iter();
            loop {
                match (left_iter.next(), right_iter.next()) {
                    (None, None) => return Smaller::Equal,
                    (None, Some(_)) => return Smaller::Left,
                    (Some(_), None) => return Smaller::Right,
                    (Some(left_el), Some(right_el)) => {
                        let inner_compare = is_left_smaller(left_el, right_el);
                        if inner_compare != Smaller::Equal {
                            return inner_compare
                        }
                    }
                }
            }
        }
        (Element::Int(left_int), Element::List(_)) => is_left_smaller(
            &Element::List(vec![Element::Int(*left_int)]), right
        ),
        (Element::List(_), Element::Int(right_int)) => is_left_smaller(
            left, &Element::List(vec![Element::Int(*right_int)])
        )
    }
}

impl From<&str> for Element {
    fn from(data: &str) -> Self {
        if data.starts_with("[") {
            let mut list_data = vec![];
            let mut current_str: String = "".to_string();
            let mut current_open = -1;
            for car in data.chars() {
                match car {
                    '[' => {
                        current_open+= 1;
                        if current_open > 0 {
                            current_str = format!("{}{}", current_str, car)
                        }
                    },
                    ']' => {
                        current_open-= 1;
                        if current_open >= 0 {
                            current_str = format!("{}{}", current_str, car)
                        } else if current_str.len() > 0 {
                            list_data.push(Element::from(current_str.as_str()));
                        }
                    },
                    ',' if current_open == 0 => {
                        list_data.push(Element::from(current_str.as_str()));
                        current_str = "".to_string();
                    },
                    _ => current_str = format!("{}{}", current_str, car)
                }
            }
            Element::List(list_data)
        } else {
            Element::Int(data.parse().unwrap())
        }
    }
}

pub fn day13() {
    let elements = include_str!("day13.txt").lines().collect::<Vec<_>>().chunks(3)
        .map(|data| {
            let left = data[0];
            let right = data[1];
            (Element::from(left), Element::from(right))
        }).collect::<Vec<_>>();

    let sum: u32 = elements.iter()
        .enumerate().filter(|(_, (left, right))|
        is_left_smaller(left, right) == Smaller::Left
    ).map(|(index, _)| (index + 1) as u32).sum();

    println!("Step 1 : {}", sum);

    let mut elements: Vec<_> = elements.into_iter().flat_map(|(left, right)| vec![left, right])
        .map(|el| (el, false))
        .chain(vec![
            (Element::List(vec![Element::List(vec![Element::Int(2)])]), true),
            (Element::List(vec![Element::List(vec![Element::Int(6)])]), true),
        ]).collect();


    elements.sort_by(|(left, _), (right, _)| {
        match is_left_smaller(left, right) {
            Smaller::Left => Ordering::Less,
            Smaller::Right => Ordering::Greater,
            Smaller::Equal => Ordering::Equal
        }
    });
    let mut spe_indices = elements.into_iter().enumerate().filter(|(_, (_, is_spe))| *is_spe)
        .map(|(index, _)| (index + 1) as u32);

    let step_2_result = spe_indices.next().unwrap() * spe_indices.next().unwrap();

    println!("Step 2 : {}", step_2_result)

}