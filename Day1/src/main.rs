// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_mut, unused_variables)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {

    let path = Path::new("calibration.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut sum: u32 = 0;

    let valid_numbers: [&str; 20] = ["zero", "one",  "two", "three", "four", "five", "six", "seven", "eight","nine",
                                     "0",    "1",    "2",   "3",     "4",    "5",    "6",   "7",     "8",    "9"];

    for line in reader.lines() {
        let line = line?;
        // println!("{:?}", line);
        // get first value that appears from left to right
        let first: u32 = get_first(line.as_str(), &valid_numbers);
        // println!("first {:?}", first);
        // get first value that appears from right to left
        let last: u32 = get_last(line.as_str(), &valid_numbers);
        // println!("last {:?}", last);
        // use both values to build the correct calibration
        let result: u32 = first*10 + last;
        // println!("Result {:?}", result);
        // add current calibration to total sum
        sum = sum + result;
    }
    // print answer
    println!("The sum of all of the calibration values is {}", sum);

    // for line in reader.lines() {
    //     let line = line?;
    //     println!("{:?}", line);
    //     // remove non numbers
    //     curr_calib_str = line.chars().filter(|c| c.is_digit(10)).collect();
    //     // if string length > 2, get only first and last digit
    //     if curr_calib_str.len() > 2 {
    //         let first = curr_calib_str.chars().next().unwrap();
    //         let last = curr_calib_str.chars().last().unwrap();
    //         result = format!("{}{}", first, last);
    //         curr_calib = result.parse::<u32>().unwrap();
    //     }
    //     else if curr_calib_str.len() == 1 {
    //         let first = curr_calib_str.chars().next().unwrap();
    //         result = format!("{}{}", first, first);
    //         curr_calib = result.parse::<u32>().unwrap();
    //     }
    //     else {
    //         curr_calib = curr_calib_str.parse::<u32>().unwrap();
    //     }
    //     println!("{}", curr_calib);
    //     // add to sum
    //     calib_total += curr_calib;
    // }
    // // print answer
    // println!("The sum of all of the calibration values is {}", calib_total);
    
    return Ok(());
}

fn get_first(line: &str, valid: &[&str]) -> u32 {
    let mut pos: usize = 99999;
    let mut index: u32 = 0;
    // for each member in the valid array
    for m in 0..valid.len() {
        // try to find the member in the original line
        let result = line.find(valid[m]);
        // if a match was found and its position is smaller than previously smaller
        if (result != None) && (result <= Some(pos)) {
            // save the new smallest position
            pos = result.unwrap();
            // get the index of the matching string
            index = m as u32;
            // use the index of the matching string to find the numerical value
            index = index%10;
        }
    }
    return index;
}

fn get_last(line: &str, valid: &[&str]) -> u32 {
    let mut pos: usize = 0;
    let mut index: u32 = 0;
    // for each memer in the valid array
    for m in 0..valid.len() {
        // try to find the member in the original line
        let result = line.rfind(valid[m]);
        // if valid was found and its position is larger than previously larger
        if (result != None) && (result >= Some(pos)) {
            // save the new largest position
            pos = result.unwrap();
            // get the index of the matching string
            index = m as u32;
            // use the index of the matching string to find the numerical value
            index = index%10;
        }
    }
    return index;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
