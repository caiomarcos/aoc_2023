// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_mut, unused_variables)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {

    let path = Path::new("games.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut sum: u32 = 0;

    for line in reader.lines() {
        let mut power: u32 = 0;
        let mut fail = false;
        // extract the string
        let line = line?;
        // split into the game id part and the game results part
        let orig_parts = line.split(": ");
        // put parts into vector
        let line_vector = orig_parts.collect::<Vec<&str>>();
        // first part contains game id
        let game_id = line_vector[0];
        let game_id_vec = game_id.split(" ").collect::<Vec<&str>>();
        // second part contains game results
        let games = line_vector[1];
        // split games into the different games
        let games_parts = games.split("; ");
        // println!("Game ID {}",game_id_vec[1].parse::<u32>().unwrap());
        // for each game
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for game in games_parts {
            // println!("Game ID {}",game_id_vec[1].parse::<u32>().unwrap());
            // split into the color results
            let results = game.split(", ");
            // for each color result
            for result in results {
                let color_number = result.split(" ");
                let color_number_vec = color_number.collect::<Vec<&str>>();
                // println!{"{} {}", color_number_vec[1], color_number_vec[0]};
                if color_number_vec[1] == "red" {
                    if color_number_vec[0].parse::<u32>().unwrap() > min_red{
                        min_red = color_number_vec[0].parse::<u32>().unwrap();
                    }
                    // if color_number_vec[0].parse::<u32>().unwrap() > 12 {
                    //     // println!("Too many red balls: {}!", color_number_vec[0].parse::<u32>().unwrap());
                    //     fail = true;
                    //     break;
                    // }
                    
                }
                else if color_number_vec[1] == "green" {
                    if color_number_vec[0].parse::<u32>().unwrap() > min_green{
                        min_green = color_number_vec[0].parse::<u32>().unwrap();
                    }
                    // if color_number_vec[0].parse::<u32>().unwrap() > 13 {
                    //     // println!("Too many green balls: {}!", color_number_vec[0].parse::<u32>().unwrap());
                    //     fail = true;
                    //     break;
                    // }
                }
                else if color_number_vec[1] == "blue" {
                    if color_number_vec[0].parse::<u32>().unwrap() > min_blue{
                        min_blue = color_number_vec[0].parse::<u32>().unwrap();
                    }
                    // if color_number_vec[0].parse::<u32>().unwrap() > 14 {
                    //     // println!("Too many blue balls: {}!", color_number_vec[0].parse::<u32>().unwrap());
                    //     fail = true;
                    //     break;
                    // }  
                }
            }
        }
        power = min_red*min_green*min_blue;
        sum += power;
        // if !fail {
        //     // println!("Game OK");
        //     sum += game_id_vec[1].parse::<u32>().unwrap();
        //     // println!("Total so far {}",sum);
        // }
        // else {
        //     // println!("Game failed");
        //     // println!("Total so far {}",sum);
        // }
    }
    // print answer
    // println!("The sum of the IDs of the games is {:?}", sum);
    println!("The sum of the power of these sets is {:?}", sum);
    
    return Ok(());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
