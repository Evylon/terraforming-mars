use terraforming_mars::convert_csv;

const CARD_FOLDER: &str = "cards/";
const CARD_LIST: &str = "card_list.csv";

fn main() {
    convert_csv(CARD_LIST.to_owned(), CARD_FOLDER.to_owned());
}
