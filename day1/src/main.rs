use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::zip;

fn main() {
    fn part1() -> () {
        let path = "src/input/input.txt";
        let mut right: Vec<i32> = vec![];
        let mut left: Vec<i32> = vec![];
        read_to_string(path).unwrap().lines().for_each(|line| {
            let mut chunks = line.split_ascii_whitespace();
            right.push(chunks.next().unwrap().parse().unwrap());
            left.push(chunks.next().unwrap().parse().unwrap());
        });

        right.sort();
        left.sort();

        let result = zip(right, left).fold(0, |acc, (r, l)| acc + (r - l).abs());
        println!("Result part 1: {}", result);
    }
    fn part2() -> () {
        let path = "src/input/input2.txt";
        let mut right: Vec<i32> = vec![];
        let mut left: Vec<i32> = vec![];
        read_to_string(path).unwrap().lines().for_each(|line| {
            let mut chunks = line.split_ascii_whitespace();
            right.push(chunks.next().unwrap().parse().unwrap());
            left.push(chunks.next().unwrap().parse().unwrap());
        });

        let mut map_right = HashMap::new();
        let mut map_left = HashMap::new();
        for x in &right {
            *map_right.entry(x).or_insert(0) += 1;
        }

        for x in &left {
            *map_left.entry(x).or_insert(0) += 1;
        }

        let result = map_right.iter().fold(0, |acc, (&k, v)| {
            let left = map_left.get(k).unwrap_or(&0);
            acc + left * k
        });

        println!("Result part two: {}", result);
    }

    part1();
    part2();
}
