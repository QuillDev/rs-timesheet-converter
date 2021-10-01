pub(crate) fn to_updated_sheet_vec(original_vec: Vec<Vec<String>>) -> Vec<Vec<String>> {
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
pub(crate) fn read_csv_file(path: &str) -> Vec<Vec<String>> {
    let response = std::fs::read_to_string(path);
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


/// Prints out a sheets as represented by a reference to a 2d vector containing string values
/// # Arguments
/// * `sheets` - The sheets to print out to the console
pub(crate) fn print_sheet(sheet: &Vec<Vec<String>>) {
    for (idx, row) in sheet.iter().enumerate() {
        print!("(Row: {}) -> ", idx + 1);
        for col in row {
            print!(" {}", *col)
        }
        println!();
    }
}
