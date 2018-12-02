use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;
use std::collections::HashMap;
use std::string::String;
use std::cmp::min;
use std::ops::Range;


fn get_twos_and_threes(id: &str) -> (u64, u64) {
    let mut ids: HashMap<char, u64> = HashMap::new();
    for c in id.chars() {
        if ids.contains_key(&c) {
            let mut count = *ids.get(&c).unwrap();
            count += 1;
            ids.insert(c, count);
        } else {
            ids.insert(c, 1);
        }
    }

    let twos = ids.values().filter(|&count| *count == 2).count();
    let threes = ids.values().filter(|&count| *count == 3).count();

    return (min(twos as u64, 1), min(threes as u64, 1));
}

fn differences(lhs: &str, rhs: &str) -> u64 {
    let mut differences: u64 = 0;
    for (c_lhs, c_rhs) in lhs.chars().zip(rhs.chars()) {
        if c_lhs != c_rhs {
            differences += 1;
        }
    }
    return differences;
}

fn part_one(values: &Vec<&str>) {
    let mut twos_total = 0;
    let mut threes_total = 0;
    for id in values {
        let (twos, threes) = get_twos_and_threes(id);
        twos_total += twos;
        threes_total += threes;
    }

    println!("{} * {} = {}", twos_total, threes_total, twos_total * threes_total);
}

fn part_two(values: &Vec<&str>) {
    println!("There are {} elements", values.len());

    let mut matrix: Vec<Vec<u64>> =
        (0..(values.len())).map(|_| (0..(values.len())).map(|_| 0).collect()).collect();

    for (index_lhs, id_lhs) in values.iter().enumerate() {
        for (index_rhs, id_rhs) in values.iter().enumerate() {
            matrix[index_lhs][index_rhs] = differences(id_lhs, id_rhs);
        }
    }

    for index_lhs in (Range{start: 0, end: values.len()}) {
        for index_rhs in (Range{start: index_lhs + 1, end: values.len()}) {
            if matrix[index_lhs][index_rhs] == 1 {
                let lhs = values[index_lhs];
                let rhs = values[index_rhs];

                let mut result = String::new();
                for (c_lhs, c_rhs) in lhs.chars().zip(rhs.chars()) {
                    if c_lhs == c_rhs {
                        result.push(c_lhs);
                    }
                }

                println!("LHS    = {}, RHS = {}", lhs, rhs);
                println!("Result = {}", result);
            }
        }
    }
}

fn main() {
    let filename = "src/day_2/input.txt";
    let mut file = File::open(filename).expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let values: Vec<&str> = contents.split_whitespace().collect();

    part_one(&values);
    part_two(&values);

}
