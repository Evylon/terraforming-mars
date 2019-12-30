use std::env;

mod card;

// The `main` function is where your program starts executing.
fn main() {
    // Create a CSV parser that reads data from stdin.
    let mut path = env::current_dir().expect("current working directory");
    // FIXME column Req: Venus is duplicate. Good practive would be to rename headers after loading
    path.push("card_list.csv");
    let mut rdr = csv::Reader::from_path(path).expect("csv reader");
    // let columns = rdr.headers().expect("columns of card list");
    // println!("{:?}", columns);
    for result in rdr.deserialize() {
        let csv_card: card::CSVCard = result.expect("a csvCard");
        let card = card::Card::from(csv_card);
        println!("{:?}", card);
    }
}
