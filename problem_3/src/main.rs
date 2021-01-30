use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let lines = lines_from_file("rsc/input.txt").expect("error");

    let solution_2 = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    .iter().fold(1, |acc, el| acc * Mountain{x_step: el.0, y_step: el.1, lines: lines.clone()}.count(|ch| *ch == '#' ));

    println!("{:?}", solution_2);
}

trait Count<T> {
    fn count_with_index(self, func: impl Fn(&T, usize) -> bool) -> i32;
    fn count(self, func: impl Fn(&T) -> bool) -> i32;
}

// impl<T> Count<T> for Vec<T> {
//     fn count_with_index(self, func: impl Fn(&T, usize) -> bool) -> i32 {
//         let mut counter = 0;
    
//         for (index, element) in self.iter().enumerate() {
//             if func(&element, index) {counter += 1};
//         }
    
//         counter
//     }

//     fn count(self, func: impl Fn(&T) -> bool) -> i32 {
//         let mut counter = 0;
    
//         for (_index, element) in self.iter().enumerate() {
//             if func(&element) {counter += 1};
//         }
    
//         counter
//     }
// }


impl Count<char> for Mountain<String> {
    fn count_with_index(self, func: impl Fn(&char, usize) -> bool) -> i32 {
        let mut counter = 0;
    
        for (index, element) in self.into_iter().enumerate() {
            if func(&element, index) {counter += 1};
        }
    
        counter
    }
    fn count(self, func: impl Fn(&char) -> bool) -> i32 {
        let mut counter = 0;
    
        for (_index, element) in self.into_iter().enumerate() {
            if func(&element) {counter += 1};
        }
    
        counter
    }
}

impl IntoIterator for Mountain<String> {
    type Item = char;
    type IntoIter = MountainIntoItorator;

    fn into_iter(self) -> Self::IntoIter {
        MountainIntoItorator {
            mountain: self,
            x_index: 0,
            y_index: 0,
        }
    }
}

#[derive(Debug)]
struct MountainIntoItorator {
    mountain: Mountain<String>,
    x_index: usize,
    y_index: usize,
}

#[derive(Debug)]
struct Mountain<String> {
    x_step: usize,
    y_step: usize,
    lines: Vec<String>,
}


impl Iterator for MountainIntoItorator {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        self.y_index += self.mountain.y_step;
        self.x_index += self.mountain.x_step;

        if  self.y_index < self.mountain.lines.len() {
            Some(self.mountain.lines[self.y_index].chars().nth(self.x_index % self.mountain.lines[0].len()).unwrap())
        } else {
            None
        }
    }
}