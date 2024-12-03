use regex::Regex;

#[derive(Debug, PartialEq)]
enum DODONT {
    DO,
    DONT,

}

pub fn part2(mem: String) {

    let reg = Regex::new(r"(don't\(\).|do\(\).|.?.?.?.?.?)mul\(([0-9]+),([0-9]+)\)").unwrap();
    let do_mul = Regex::new(r"do\(\).").unwrap();
    let dont_mul = Regex::new(r"don't\(\).").unwrap();

    // let mut prereged = vec![];
    let mut reged = vec![];

    let mut do_dont = DODONT::DO;
    
    for (_, [dodont, num1, num2]) in reg.captures_iter(mem.as_str()).map(|c| c.extract()) {
        // println!("DODONT: {}, num1 {}, num2 {}", dodont, num1, num2);
        reged.push((dodont, num1.parse::<i32>().unwrap(), num2.parse::<i32>().unwrap()));
    };

    // println!("{:?}", reged);

    let mut total:i32 = 0;
  

    for tup in reged {
        let ddnt = tup.0;
        if dont_mul.is_match(ddnt) {
            do_dont = DODONT::DONT
        }else if do_mul.is_match(ddnt) {
            do_dont = DODONT::DO
        }else {
            do_dont = do_dont
        }
    println!("DDNT: {}, num1 {}, num2 {}", tup.0, tup.1, tup.2);
    println!("DODONT {:?}", do_dont);

        if do_dont == DODONT::DO {
        total += tup.1 * tup.2
        }
    
    println!("Running Total: {}", total)
    }

    println!("Total: {}", total)


}