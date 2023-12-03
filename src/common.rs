use std::fs;

/// Get data from a .txt file or string.
///
/// This function checks if the input is a path by checking if it ends in .txt. If it does not end
/// in .txt then the function will return the input data. This function will panic if it fails to
/// read the .txt file.
pub fn get_data_from_file_or_str(path_or_data: String) -> String {
    if path_or_data.ends_with(".txt") {
         fs::read_to_string(path_or_data).expect("Error reading data file")
    } else {
        path_or_data
    }
}