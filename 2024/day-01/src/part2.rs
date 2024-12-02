use std::usize;



pub fn part2(input: &String) -> usize{

    let mut row1 = vec![];
    let mut row2 = vec![];
    
    for line in input.as_str().lines() {
        let mut items = line.split_whitespace();
        row1.push(
            items.next().unwrap().parse::<usize>().unwrap(),
        );
        row2.push(
            items.next().unwrap().parse::<usize>().unwrap(),
        );
    }

    let mut total_diff: usize = 0;

    let result: usize = row1.iter().map(|num1| {num1 * row2.iter().filter(|r| &num1 == r).count()}).sum();

    total_diff +=  result;
    
    total_diff

    // for num1 in &row1 {
    //     let mut num_times:i32 = 0;

    //     for num2 in &row2 {
    //         if num1 == num2 {
    //             num_times += 1;
    //         }
    //     }
    //     let num_total_diff = num1 * num_times;
    //     total_diff +=  num_total_diff;
    //     // println!("r1: {},  Times: {}, Times Diff: {}, Total: {}", num1, num_times, num_total_diff, total_diff);
    // }
    

}