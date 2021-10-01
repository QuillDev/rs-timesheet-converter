use std::fs::read_to_string;

fn main() {
    let sheet_path = "/home/quill/test.csv";
    let contents = read_csv_file(sheet_path);
    if contents.is_empty() {
        println!("Contents are empty for path");
        return;
    }

    let new_sheet = to_updated_sheet_vec(read_csv_file(sheet_path));
    print_sheet(&new_sheet)
}

fn to_updated_sheet_vec(original_vec: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut new_sheet: Vec<Vec<String>> = Vec::new();

    for old_row in original_vec {
        let mut new_row: Vec<String> = Vec::new();
        for (col_idx, old_col) in old_row.iter().enumerate() {
            let mut value = old_col.to_string();
            if col_idx == 0 {
                value = String::from("-1");
            }
            new_row.push(value);
        }
        new_sheet.push(new_row);
    }

    return new_sheet;
}


/// Reads a CSV file from the given path and returns a 2d vector containing the
/// data values as strings
/// # Arguments
/// * `path` - Path to the csv file
fn read_csv_file(path: &str) -> Vec<Vec<String>> {
    let response = read_to_string(path);
    if response.is_err() {
        println!("{}", response.err().unwrap());
        return Vec::new();
    }
    return to_sheet_vector(&response.unwrap());
}

fn to_sheet_vector(data: &str) -> Vec<Vec<String>> {
    return data.split('\n')
        .map(|word| {
            word.split(",")
                .map(|word| { String::from(word) })
                .collect()
        })
        .collect();
}


/// Prints out a sheet as represented by a reference to a 2d vector containing string values
/// # Arguments
/// * `sheet` - The sheet to print out to the console
fn print_sheet(sheet: &Vec<Vec<String>>) {
    for (idx, row) in sheet.iter().enumerate() {
        print!("(Row: {}) -> ", idx + 1);
        for col in row {
            print!(" {}", *col)
        }
        println!();
    }
}
