use anyhow::Result;
use itertools::Itertools;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

#[derive(Clone)]
enum Part {
    One,
    Two,
}

enum Input {
    Test,
    Puzzle,
}

struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> usize>,
    inspect_count: u64,
    part: Part,
    part2_keep_value_low: Option<Box<dyn Fn(u64) -> u64>>,
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .finish()
    }
}

impl Monkey {
    fn inspect(&mut self, index: usize) -> usize {
        self.inspect_count += 1;
        // println!("current value: {}", self.items[index]);
        self.items[index] = (self.operation)(self.items[index]);
        match self.part {
            Part::One => self.items[index] /= 3,
            Part::Two => {
                self.items[index] = (self.part2_keep_value_low.as_ref().unwrap())(self.items[index])
            }
        }
        (self.test)(self.items[index])
    }

    fn step(&mut self) -> Vec<usize> {
        let mut receivers: Vec<usize> = Vec::new();
        for i in 0..self.items.len() {
            let recv = self.inspect(i);
            receivers.push(recv);
        }
        receivers
    }

    fn remove_item(&mut self, index: usize) -> u64 {
        self.items.remove(index)
    }

    fn add_item(&mut self, item: u64) {
        self.items.push(item);
    }
}

fn round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        let receivers = monkeys[i].step();
        let num_thrown = monkeys[i].items.len();
        for j in 0..num_thrown {
            let item = monkeys[i].remove_item(0); // next item to throw is always currently first
            let recv = receivers[j];
            monkeys[recv].add_item(item)
        }
    }
}

fn solve(part: Part, input: Input) -> Result<u64> {
    let file = match input {
        Input::Test => File::open("test.txt")?,
        Input::Puzzle => File::open("input.txt")?,
    };
    let reader = BufReader::new(file);

    let mut all_divs: Vec<u64> = Vec::new();

    let mut monkeys: Vec<Monkey> = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .split(|line| line.is_empty())
        .map(|line_group| {
            let mut iter = line_group.iter();
            iter.next().unwrap(); // "Monkey i:"

            let items = iter.next().unwrap().split(": ").collect::<Vec<&str>>()[1];
            let items = items.split(", ").collect::<Vec<&str>>();
            let items: Vec<u64> = items.iter().map(|x| x.parse().unwrap()).collect();

            let operations = iter.next().unwrap().split("old ").collect::<Vec<&str>>()[1];
            let (op, value) = operations.split_whitespace().collect_tuple().unwrap();
            let operation: Box<dyn Fn(u64) -> u64> = match (op, value.parse::<u64>()) {
                ("*", Ok(n)) => Box::new(move |x: u64| -> u64 { x * n }),
                ("+", Ok(n)) => Box::new(move |x: u64| -> u64 { x + n }),
                ("*", Err(_)) => Box::new(|x: u64| -> u64 { x * x }),
                ("+", Err(_)) => Box::new(|x: u64| -> u64 { x + x }),
                _ => panic!(),
            };

            let test_div = iter.next().unwrap().split("by ").collect::<Vec<&str>>()[1];
            let test_div: u64 = test_div.parse().unwrap();
            all_divs.push(test_div);
            let true_receiver = iter.next().unwrap().split("monkey ").collect::<Vec<&str>>()[1];
            let true_receiver: usize = true_receiver.parse().unwrap();
            let false_receiver = iter.next().unwrap().split("monkey ").collect::<Vec<&str>>()[1];
            let false_receiver: usize = false_receiver.parse().unwrap();
            let test = Box::new(move |x: u64| -> usize {
                if (x % test_div) == 0 {
                    true_receiver
                } else {
                    false_receiver
                }
            });

            Monkey {
                items,
                operation,
                test,
                inspect_count: 0,
                part: part.clone(),
                part2_keep_value_low: None, // will be set later
            }
        })
        .collect();

    // not technically LCM but good enough
    let mut iterator = all_divs.into_iter();
    let first_div = iterator.next().unwrap();
    let lcm = iterator.fold(first_div, |product, next_div| product * next_div);

    for monkey in &mut monkeys {
        monkey.part2_keep_value_low = Some(Box::new(move |x: u64| -> u64 { x % lcm }));
    }

    println!("before round 1:");
    println!("{:?}", monkeys);

    let num_rounds = match part {
        Part::One => 20,
        Part::Two => 10000,
    };

    for _ in 0..num_rounds {
        round(&mut monkeys);
    }

    println!("after all rounds:");
    println!("{:?}", monkeys);

    for i in 0..monkeys.len() {
        println!(
            "Monkey {} inspected items {} times.",
            i, monkeys[i].inspect_count
        );
    }

    let mut inspect_counts: Vec<u64> = monkeys.iter().map(|m| m.inspect_count).collect();
    inspect_counts.sort();
    inspect_counts.reverse();

    let result = inspect_counts[0] * inspect_counts[1];

    Ok(result)
}

fn main() -> Result<()> {
    let start = Instant::now();

    let test1 = solve(Part::One, Input::Test)?;
    assert_eq!(test1, 10605);

    let result1 = solve(Part::One, Input::Puzzle)?;
    println!("part 1 result: {}", result1);

    let test2 = solve(Part::Two, Input::Test)?;
    assert_eq!(test2, 2713310158);

    let result2 = solve(Part::Two, Input::Puzzle)?;
    println!("part 2 result: {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}
