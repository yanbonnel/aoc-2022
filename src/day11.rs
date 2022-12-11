use std::collections::HashMap;
use std::ops::{Add, Mul};

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: ThrowTest,
    inspects: u64,
}

#[derive(Clone, Debug)]
enum OperationMember {
    Old,
    Numer(u64),
}

impl OperationMember {
    fn new(line: &str) -> OperationMember {
        if line == "old" {
            OperationMember::Old
        } else {
            OperationMember::Numer(line.parse().unwrap())
        }
    }

    fn get(&self, item: u64) -> u64 {
        match self {
            OperationMember::Old => item.clone(),
            OperationMember::Numer(number) => *number
        }
    }
}

#[derive(Clone, Debug)]
struct Operation {
    left: OperationMember,
    sign: char,
    right: OperationMember,
}

impl Operation {
    pub fn new(line: &str) -> Operation {
        let mut words = line.split(" ");
        Operation {
            left: OperationMember::new(words.next().unwrap()),
            sign: words.next().unwrap().chars().next().unwrap(),
            right: OperationMember::new(words.next().unwrap()),
        }
    }

    pub fn apply(&self, item: u64) -> u64 {
        let left = self.left.get(item);
        let right = self.right.get(item);
        match self.sign {
            '*' => left * right,
            '+' => left + right,
            _ => panic!()
        }
    }
}

#[derive(Clone, Debug)]
struct ThrowTest {
    divisible_by: u64,
    monkey_true: usize,
    monkey_false: usize,
}

impl ThrowTest {
    fn monkey(&self, item: u64) -> usize {
        if item % self.divisible_by == 0 {
            self.monkey_true
        } else {
            self.monkey_false
        }
    }
}

pub fn day11() {
    let mut acc: (Vec<u64>, Option<Operation>, Option<ThrowTest>) = (vec![], None, None);

    let mut monkeys_step1 = vec![];
    for line in include_str!("day11.txt").lines() {
        if line.is_empty() {
            let (items, operation, test) = acc.clone();
            acc = (vec![], None, None);
            monkeys_step1.push(Monkey {
                items,
                operation: operation.unwrap(),
                test: test.unwrap(),
                inspects: 0,
            })
        } else if line.starts_with("Monkey") {
            // skip
        } else {
            let mut words = line.split(": ");
            let order = words.next().unwrap().trim_start();
            let order_content = words.next().unwrap();
            match order {
                "Starting items" => {
                    acc.0 = order_content.split(", ")
                        .map(|number| number.parse().unwrap())
                        .collect::<Vec<_>>();
                }
                "Operation" => {
                    let op_line = order_content.split(" = ")
                        .skip(1).next().unwrap();
                    acc.1 = Some(Operation::new(op_line));
                }
                "Test" => {
                    acc.2 = Some(ThrowTest {
                        divisible_by: order_content.split(" ").last().unwrap().parse().unwrap(),
                        monkey_true: 0,
                        monkey_false: 0,
                    })
                }
                "If true" => {
                    acc.2.as_mut().unwrap().monkey_true = order_content.split(" ").last().unwrap().parse().unwrap();
                }
                "If false" => {
                    acc.2.as_mut().unwrap().monkey_false = order_content.split(" ").last().unwrap().parse().unwrap();
                }
                _ => panic!()
            }
        }
    }
    let (items, operation, test) = acc.clone();
    acc = (vec![], None, None);
    monkeys_step1.push(Monkey {
        items,
        operation: operation.unwrap(),
        test: test.unwrap(),
        inspects: 0,
    });

    let mut monkeys_step2 = monkeys_step1.clone();

    for _ in 0..20 {
        for index in 0..monkeys_step1.len() {
            let items = monkeys_step1[index].items.clone();
            monkeys_step1[index].items = vec![];

            for item in items.iter() {
                let item = monkeys_step1[index].operation.apply(item.clone()) / 3;
                let monkey_to_throw = monkeys_step1[index].test.monkey(item);
                monkeys_step1[monkey_to_throw].items.push(item);
            }

            monkeys_step1[index].inspects += items.len() as u64;
        }
    }

    let mut inspects = monkeys_step1.iter().map(|monkey| monkey.inspects)
        .collect::<Vec<_>>();
    inspects.sort();
    let result = inspects.pop().unwrap() * inspects.pop().unwrap();

    println!("Step 1 : {}", result);

    let modulus: u64 = monkeys_step2.iter().map(|monkey: &Monkey| monkey.test.divisible_by)
        .product();

    for _ in 0..10000 {
        for index in 0..monkeys_step2.len() {
            let items = monkeys_step2[index].items.clone();
            monkeys_step2[index].items = vec![];

            for item in items.iter() {
                let item = monkeys_step2[index].operation.apply(item.clone()) % modulus;
                let monkey_to_throw = monkeys_step2[index].test.monkey(item);
                monkeys_step2[monkey_to_throw].items.push(item);
            }

            monkeys_step2[index].inspects += items.len() as u64;
        }
    }

    let mut inspects = monkeys_step2.iter().map(|monkey| monkey.inspects)
        .collect::<Vec<_>>();
    inspects.sort();
    let result = inspects.pop().unwrap() * inspects.pop().unwrap();

    println!("Step 2 : {}", result);
}