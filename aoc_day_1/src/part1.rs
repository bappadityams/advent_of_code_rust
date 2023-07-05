use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(input_filename) {
        // Consumes the iterator, returns an (Optional) String
        let mut game_score: u32 ;
        let mut total_score: u32 = 0;

        for line in lines {

            if let Ok(ip) = line {
                if ip.len() == 0 {
                    continue;           
                }
            let outcomes: Vec<&str> = ip.split_whitespace().collect::<Vec<&str>>();    
            
            game_score = calculatewinning_score(&outcomes);
    
            total_score += game_score;

                             
            }
            
        }
        println!("Total score :{}", total_score);  
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calculatewinning_score(outcomes: &[&str]) -> u32 {


    let mut score: u32 = 0;

    if outcomes[1] == "X" {
        score = 1;
    } else if outcomes[1] == "Y" {
        score = 2;
    } else if outcomes[1] == "Z" {
        score = 3;
    } 
    
    if outcomes[0] == "A" && outcomes[1] == "X"  {
        score += 3;
    } else if outcomes[0] == "A" && outcomes[1] == "Y" {
        score += 6;
    } else if outcomes[0] == "A" && outcomes[1] == "Z"  {
        score += 0;
    } else if outcomes[0] == "B" && outcomes[1] == "X" {
        score += 0;
    } else if outcomes[0] == "B" && outcomes[1] == "Y" {
        score += 3;
    } else if outcomes[0] == "B" && outcomes[1] == "Z" {
        score += 6;
    } else if outcomes[0] == "C" && outcomes[1] == "X" {
        score += 6;
    }else if outcomes[0] == "C" && outcomes[1] == "Y"  {
        score += 0;
    } else if outcomes[0] == "C" && outcomes[1] == "Z" {
        score += 3;
    }
    return score;
}
