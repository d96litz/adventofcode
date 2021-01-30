use std::fs;

fn main(){
    let mut nums = fs::read_to_string("rsc/input.txt")
    .expect("Something went wrong reading the file")
    .split("\n")
    .map(|e| e.parse().unwrap() )
    .collect::<Vec<i32>>();

    nums.sort();

    let result = search_value(&nums, 2020);

    println!("The winners are {:?}", result);
}

fn search_value(vals: &Vec<i32>, sum: i32) -> (i32, i32, i32) { 
    for &val1 in vals {
        if val1 > sum {break};
        let remainder = sum - val1;
        
        for &val2 in vals {
            if val2 > remainder {break};
            let remainder = remainder - val2;

            for &val3 in vals {
                if val3 > remainder {break};
                let remainder = remainder - val3;

                if remainder == 0 {return (val1, val2, val3);}
            }
        }
    }
    
    return (0, 0, 0);
}