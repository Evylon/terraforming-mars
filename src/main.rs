use std::env;
use std::process;

// The `main` function is where your program starts executing.
fn main() {
    // Create a CSV parser that reads data from stdin.
    let mut path = env::current_dir().expect("current working directory");
    path.push("card_list.csv");
    let mut rdr = csv::Reader::from_path(path).expect("csv reader");
    let columns = rdr.headers().expect("columns of card list");
    println!("{:?}", columns);
    process::exit(0);
}
