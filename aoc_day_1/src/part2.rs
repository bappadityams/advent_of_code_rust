use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, vec};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(input_filename) {
        // Consumes the iterator, returns an (Optional) String
        let mut tallies: Vec<u32> = vec![0];
        

        for line in lines {
            if let Ok(ip) = line {
                if ip.len() == 0 {
                    
                    tallies.push(0);
                    continue;           
                }
                let calorie= ip.parse::<u32>().unwrap();
                *tallies.last_mut().unwrap() += calorie;
                             
            }
        }
        tallies.sort();
        println!("Total of highest 3 tally:{}", tallies.pop().unwrap()+tallies.pop().unwrap()+tallies.pop().unwrap());   
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}