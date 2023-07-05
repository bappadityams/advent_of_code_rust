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
            let mut outcomes: Vec<&str> = ip.split_whitespace().collect::<Vec<&str>>();    
            
            game_score = calculatewinning_score(&mut outcomes);
    
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

fn calculatewinning_score(outcomes: &mut Vec<&str>) -> u32 {


    let mut score: u32 = 0;

    let mut subject_move = outcomes[1];


    if subject_move=="X" {
    
        if outcomes[0] == "A"{
            subject_move = "S";
        } else if outcomes[0] == "B" {
            subject_move = "R";
        } else if outcomes[0] == "C" {
            subject_move = "P";

        }
    }

    if subject_move=="Y" {
    
        if outcomes[0] == "A" {
            subject_move = "R";
        } else if outcomes[0] == "B" {
            subject_move = "P";
        } else if outcomes[0] == "C" {
            subject_move = "S";

        }
    }

    if subject_move=="Z" {
    
        if outcomes[0] == "A" {
            subject_move = "P";
        } else if outcomes[0] == "B" {
            subject_move = "S";
        } else if outcomes[0] == "C" {
            subject_move = "R";

        }
    }
   
    if subject_move == "R" {
        score = 1;
    } else if subject_move == "P" {
        score = 2;
    } else if subject_move== "S" {
        score = 3;
    } 

    
    if outcomes[0] == "A" && subject_move == "R"  {
        score += 3;
    } else if outcomes[0] == "A" && subject_move == "P" {
        score += 6;
    } else if outcomes[0] == "A" && subject_move == "S"  {
        score += 0;
    } else if outcomes[0] == "B" && subject_move == "S" {
        score += 6;
    } else if outcomes[0] == "B" && subject_move == "P" {
        score += 3;
    } else if outcomes[0] == "B" && subject_move == "R" {
        score += 0;
    } else if outcomes[0] == "C" && subject_move == "R" {
        score += 6;
    }else if outcomes[0] == "C" && subject_move == "P"  {
        score += 0;
    } else if outcomes[0] == "C" && subject_move == "S" {
        score += 3;
    }
    return score;
}
