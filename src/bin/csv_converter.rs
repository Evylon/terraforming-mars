use terraforming_mars::card;

const CARD_FOLDER: &str = "cards/";
const CARD_LIST: &str = "card_list.csv";

fn main() {
    card::convert_csv(CARD_LIST.to_owned(), CARD_FOLDER.to_owned());
}
