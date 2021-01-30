use regex::Regex;
use std::fs;

fn main() {
    let file_content = fs::read_to_string("rsc/input.txt")
        .expect("Something went wrong reading the file");

    println!("{:?}", part1(&file_content));
    println!("{:?}", part2(&file_content));
}

fn part1(file_content: &String) -> usize {
    return Regex::new(r"(\d+)\-(\d+) ([a-z]{1}): ([a-z]+)").unwrap()
    .captures_iter(&file_content).filter( |cap|
        (cap[1].parse::<i32>().unwrap()..cap[2].parse::<i32>().unwrap() + 1)
        .contains(&(cap[4].matches(&cap[3]).count() as i32))
    ).count();
}

fn part2(file_content: &String) -> i32 {
    let mut counter: i32 = 0;
    
    for capture in Regex::new(r"(\d+)\-(\d+) ([a-z]{1}): ([a-z]+)").unwrap().captures_iter(file_content) {
        let char_vector: Vec<char> = capture[4].chars().collect();
        let index1: usize = capture[1].parse().unwrap();
        let index2: usize = capture[2].parse().unwrap();
        // println!("{:?}", capture);
        // println!("{:?}", char_vector[index1 - 1]);
        // println!("{:?}", char_vector[index2 - 1]);
        // println!("{:?}", [char_vector[index2 - 1] as u8, char_vector[index1 - 1] as u8].iter().filter(|&el| el == &capture[3].as_bytes()[0]).count());
        // println!("{:?}", &&capture[3].as_bytes()[0]);

        if [char_vector[index2 - 1] as u8, char_vector[index1 - 1] as u8].iter().filter(|el| el == &&capture[3].as_bytes()[0] ).count() == 1 {
            counter += 1;
        }
    }

    return counter;
    // .captures_iter(&file_content).filter( |cap|
    //     let vector: Vec<char> = cap[3].chars().collect();
    //     cap[1].parse::<i32>().unwrap();
    // ).count();
}