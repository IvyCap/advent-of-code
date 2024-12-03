
use regex::Regex;
use std::{fs, path::Path};

pub fn p1_read_file() -> String{
    let part1_list = Path::new("./list.txt");

    let contents = fs::read_to_string(part1_list).expect("FILE NOT READ");

    contents

}

pub fn part1(mem: String) {

    let reg = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut reged = vec![];
    
    for (_, [num1, num2]) in reg.captures_iter(mem.as_str()).map(|c| c.extract()) {
        // println!("num1 {}, num2 {}", num1, num2);
        reged.push((num1.parse::<i32>().unwrap(), num2.parse::<i32>().unwrap()));
    };

    // println!("{:?}", reged);

    let mut total:i32 = 0;
    // let new_total = reged.iter().map(|a| {total += a.0 * a.1});
    

    // println!("New Total: {:?}", new_total)

    for tup in reged {
        total += tup.0 * tup.1
    }

    println!("Total: {}", total)


}