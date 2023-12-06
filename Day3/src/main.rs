// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_mut, unused_variables)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {

    let path = Path::new("schematic_test.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let schematic: Vec<Vec<char>> = Vec::new();
    let mut sum: u32= 0;

    for line in reader.lines() {
        let vec: Vec<char> = line.chars().collect();
        schematic.push(vec);
    }
    // print answer
    println!("The sum of all of the part numbers in the engine schematic {:?}", sum);
    
    return Ok(());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
