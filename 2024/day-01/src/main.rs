use day_01::part1::*;
use day_01::part2::*;



fn main() {

//         let input = "4 9
// 5 5
// 1 7
// 7 1
// 3 2
// 2 5"
// ;

    let file = p1_read_file();

    p1_parse_count(&file);
    // part2(input.to_string());
    part2(&file);

    println!("Part 1 Correct: 1222801, Part 2 Correct: 22545250")

}
