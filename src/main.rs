use std::fs::read_to_string;

fn main() {
    let ans = read_from_file_sachin(String::from("a.txt"));
}

fn read_from_file_sachin(file_path: String) -> Result<String, String>{
    let result = read_to_string(file_path);
    match result{
        Ok(data) => Ok(data),
        Err(err) => Err(String::from("File not found")),
    }
    }
