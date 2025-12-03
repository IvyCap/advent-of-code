mod part1;

use std::fs::read_to_string;


use part1::part1;

// use part2

fn main() {
    let mut filename = "test_data.txt";

    let mut data: Vec<String> = read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect(); // gather them together into a vector




    let count: String = part1(data);
    print!(count);

}
