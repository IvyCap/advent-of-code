use day_03::part1::*;
use day_03::part2::*;

// const TESTMEM1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

const TESTMEM2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

// const TESTMEM: &str = "mul(2,4)
// mul[3,7]
// mul(5,5)
// mul(32,64]
// mul(11,8)
// mul(8,5)";
fn main() {
    
    let file = p1_read_file();

    // part1(TESTMEM1.to_string());
    // part1(file);    
   
    
    part2(file);
    part2(TESTMEM2.to_string())
}
