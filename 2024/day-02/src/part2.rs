#[derive(PartialEq, Debug)]
enum IsSafe {
    YES,
    NO,
    MAYBE
} 

#[derive(PartialEq, Debug)]
enum IncDec {
    INC,
    DEC,
    EVEN
}

#[derive(PartialEq, Debug)]
enum REMOVED {
    YES,
    NO
}

pub fn part2(input: String) {
    let mut safe: i32 = 0;


    for line in input.as_str().lines() {
        let mut row: Vec<i32> = vec![];
        println!("line: {}", line);
        let nums = line.split_whitespace();
        println!("nums: {:?}", nums);

        for num in nums {
            row.push(num.parse::<i32>().unwrap())
        }

        let row_len = row.len();
        let mut i = 0;
        let mut safe_status :IsSafe = IsSafe::MAYBE;
        let mut removed = REMOVED::NO;

        let first_pair: IncDec = inc_dec(row[0], row[1]);
        
        while i < row_len - 1 {
            let j = i + 1;
            let diff = (row[i] - row[j]).abs();
            let next_pair: IncDec = inc_dec(row[i], row[j]);
            if first_pair != next_pair {
                match removed {
                    REMOVED::NO => 
                    REMOVED::YES => {safe_status = IsSafe::NO;
                        break}
                }
            }
            println!("i: {}, j: {}, diff: {}", row[i], row[j], diff);
            
            match diff {
                1 => safe_status = IsSafe::YES,
                2 => safe_status = IsSafe::YES,
                3 => safe_status = IsSafe::YES,
                _  => safe_status = IsSafe::NO
            }      

            println!("Safe Status: {:?}", safe_status);

            if safe_status == IsSafe::NO {
                break
        }

            i += 1
    }

        if safe_status == IsSafe::YES {
            safe += 1
}
        println!("row: {:?}", row);

    }

    println!("Safe: {}, Correct: 4", safe)
}

fn inc_dec (f: i32, s: i32) -> IncDec{
    let result: IncDec;
    if f < s {
        result = IncDec::INC
    }else if f > s  {
        result = IncDec::DEC
    }else {
        result = IncDec::EVEN
    }

    result
}