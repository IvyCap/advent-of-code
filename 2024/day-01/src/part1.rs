use std::{fs, path::Path};

pub fn p1_read_file() -> String{
    let part1_list = Path::new("./part1_list.txt");

    let contents = fs::read_to_string(part1_list).expect("FILE NOT READ");

    contents

}

pub fn p1_parse_count(input: &String) -> i32 {

let mut row1 = vec![];
let mut row2 = vec![];

for line in input.as_str().lines() {
    let mut items = line.split_whitespace();
    row1.push(
        items.next().unwrap().parse::<i32>().unwrap(),
    );
    row2.push(
        items.next().unwrap().parse::<i32>().unwrap(),
    );
}

row1.sort();
row2.sort();

let mut total_dist: i32 = 0;


let result: i32 = std::iter::zip(row1, row2).map(|(r1, r2)| {r1 - r2}.abs()).sum(); 

total_dist += result;

total_dist

// let r1_len = row1.len();

// let mut i = 0;

// while i < r1_len {
//     let diff: i32;
//     let r1_num = row1.pop().unwrap();
//     let r2_num = row2.pop().unwrap();
//     diff = (r1_num - r2_num).abs();

//     // if r1_num > r2_num {
//     //     diff = r1_num - r2_num
//     // }else {
//     //     diff = r2_num - r1_num
//     // }
//     total_dist += diff;
    
//     // println!("r1: {}, r2: {}, Diff: {}, Total: {}", r1_num, r2_num, diff, total_dist);
//     i += 1;
// }

}

