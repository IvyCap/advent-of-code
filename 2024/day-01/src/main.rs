use day_01::part1::*;
use day_01::part2::*;



fn main() {

    let file = p1_read_file();

    let ans_part1 = p1_parse_count(&file);
    let ans_part2 = part2(&file);

    println!("Part 1 Answer: {}, Correct: 1222801", ans_part1); 
    println!("Part 2 Answer: {}, Correct: 22545250", ans_part2);
}