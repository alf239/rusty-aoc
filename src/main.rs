use daggy::Dag;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    // let edges = Vec::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input_01_tst.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(wire) = line {
                let split = wire.split(" -> ");
                let vec: Vec<&str> = split.collect();
  
                println!("{} -> {}", vec[0], vec[1]);
            }
        }
    }

    // let dag = Dag::from_edges();
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
