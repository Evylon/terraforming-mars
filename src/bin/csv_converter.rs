use terraforming_mars::project;

const PROJECT_FOLDER: &str = "projects/";
const PROJECT_LIST: &str = "card_list.csv";

fn main() {
    project::convert_csv(PROJECT_LIST.to_owned(), PROJECT_FOLDER.to_owned());
}
