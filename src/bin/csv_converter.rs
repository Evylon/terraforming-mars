use terraforming_mars::patent;

const PATENT_FOLDER: &str = "patents/";
const PATENT_LIST: &str = "card_list.csv";

fn main() {
    patent::convert_csv(PATENT_LIST.to_owned(), PATENT_FOLDER.to_owned());
}
