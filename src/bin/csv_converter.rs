use terraforming_mars::{Card, CSVCard};

use std::env;
use std::fs::File;
use std::fs;
use std::io::prelude::*;

const CARD_FOLDER: &str = "cards/";
const CARD_LIST: &str = "card_list.csv";

fn main() {
    // load path
    let csv_file = CARD_LIST.to_owned();
    let out_folder = CARD_FOLDER.to_owned();
    let cwd = env::current_dir().unwrap();
    let mut card_list_path = cwd.to_owned();
    card_list_path.push(csv_file);
    let mut card_folder_path = cwd.to_owned();
    card_folder_path.push(out_folder);
    if !card_folder_path.exists() {
        fs::create_dir(card_folder_path.to_owned()).unwrap();
    }
    // load csv
    // FIXME column Req: Venus is duplicate. Good practice would be to rename headers after loading
    let mut rdr = csv::Reader::from_path(card_list_path).unwrap();
    for result in rdr.deserialize::<CSVCard>() {
        let card = Card::from(result.unwrap());
        let serialized = serde_json::to_string(&card).unwrap();
        // build output path
        let mut card_path = card_folder_path.to_owned();
        card_path.push(card.name);
        card_path.set_extension("json");
        let mut card_file = File::create(card_path).unwrap();
        card_file.write_all(serialized.as_bytes()).unwrap();
    }
}
