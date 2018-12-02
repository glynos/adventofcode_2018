use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let filename = "src/day_1/input.txt";
    let mut file = File::open(filename).expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let values: Vec<i64> = contents.split_whitespace().map(
        |value| value.parse::<i64>().unwrap()).collect();

    let mut frequency: i64 = 0;
    let mut frequencies = HashSet::new();
    let mut frequency_repeated = false;
    while !frequency_repeated {
        for value in &values {
            frequency += value;
            if frequencies.contains(&frequency) {
                frequency_repeated = true;
                break;
            }
            frequencies.insert(frequency);
        }
    }
    println!("Frequency: {}", frequency);
}
