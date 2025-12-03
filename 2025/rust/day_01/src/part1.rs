
 
pub fn part1(data: Vec<String>) -> i32 {
    let mut count = 0;
    let mut dial = 50;
    for rot in data {
        let mut dir = "x";
        dir = &rot[..0];
        let mut ticks = 00000;
        ticks = &rot[1..];

        if ticks > 100 {
            count += ticks / 100;Z
        }

    }



    return count
}