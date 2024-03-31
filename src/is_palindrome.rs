use std::io;

fn is_palindrome(s: &str) -> bool {
    return s==s.chars().rev().collect::<String>();
}

fn main() {
    println!("Enter a string:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    input = input.trim().to_string();
    if is_palindrome(&input) {
        println!("{} is PALINDROME.", input);
    } else {
        println!("{} is NOT PALINDROME.", input);
    }
}
