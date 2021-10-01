pub mod sheets;

fn main() {
    let sheet_path = "/home/quill/test.csv";
    let contents = sheets::read_csv_file(sheet_path);

    if contents.is_empty() {
        println!("Contents are empty for path");
        return;
    }

    let new_sheet = sheets::to_updated_sheet_vec(contents);
    sheets::print_sheet(&new_sheet)
}
