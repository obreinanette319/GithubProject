use std::fs;

fn main() {
    let file_path = "example.txt"; // Replace with your own file path
    let contents = fs::read_to_string(file_path).unwrap_or_else(|_| "".to_string());

    println!("File content: {}", contents);
}
