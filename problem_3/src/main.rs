use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

macro_rules! count_trees {
    (
        slide down $lines:ident with
        $($step:literal steps $dir:ident)and+
    ) => {
        {
            let mut slope = Slope{x_step: 0, y_step: 0, lines: $lines.clone()};
            $(
                let direction = stringify!($dir);
                match stringify!($dir) {
                    "right" => slope.x_step += $step,
                    "down" => slope.y_step += $step,
                    "left" => slope.x_step -= $step,
                    "up" => slope.y_step -= $step,
                    _ => panic!("Invalid direction {:?}", direction),
                }
            )*
            slope.count(|ch| *ch == '#' )
        }
    };
}

trait Count<T> {
    fn count_with_index(self, func: impl Fn(&T, usize) -> bool) -> i32;
    fn count(self, func: impl Fn(&T) -> bool) -> i32;
}

impl Count<char> for Slope<String> {
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

impl IntoIterator for Slope<String> {
    type Item = char;
    type IntoIter = SlopeIntoItorator;

    fn into_iter(self) -> Self::IntoIter {
        SlopeIntoItorator {
            slope: self,
            x_index: 0,
            y_index: 0,
        }
    }
}

#[derive(Debug)]
struct SlopeIntoItorator {
    slope: Slope<String>,
    x_index: usize,
    y_index: usize,
}

#[derive(Debug)]
struct Slope<String> {
    x_step: usize,
    y_step: usize,
    lines: Vec<String>,
}


impl Iterator for SlopeIntoItorator {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        self.y_index += self.slope.y_step;
        self.x_index += self.slope.x_step;

        if self.y_index < self.slope.lines.len() {
            Some(self.slope.lines[self.y_index].chars().nth(self.x_index % self.slope.lines[self.y_index].len()).unwrap())
        } else {
            None
        }
    }
}

fn main() {
    let lines = lines_from_file("rsc/input.txt").expect("error");
    println!("{:?}", count_trees!(
        slide down lines with
        3 steps right and
        1 steps down
    ));
}
