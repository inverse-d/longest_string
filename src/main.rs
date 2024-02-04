use std::io;

fn compare_strings(first_string: String, second_string: String) -> String {
    if first_string.trim().chars().count() > second_string.trim().chars().count() {
        let longest_string = &first_string;
        longest_string.trim().to_string()
    } else {
        let longest_string = &second_string;
        longest_string.trim().to_string()
    }
}
fn main() {
    let mut first_string = String::new();
    let mut second_string = String::new();
    println!("This program finds out which of the entered two strings is the longer one.\nEnter String 1:");
    io::stdin()
        .read_line(&mut first_string)
        .expect("Failed to read from stdin...");
    println!("Enter String 2:");
    io::stdin()
        .read_line(&mut second_string)
        .expect("Failed to read from stdin...");

    let result = compare_strings(first_string, second_string);
    println!("The longest String is: '{}' ", &result);

    
    
}
