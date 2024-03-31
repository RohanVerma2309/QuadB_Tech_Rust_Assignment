use std::io;

fn longest_common_prefix(strings: Vec<String>) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut prefix = strings[0].clone();

    for s in strings.iter().skip(1) {
        while !s.starts_with(&prefix) {
            prefix.pop();
        }
    }
    prefix
}

fn main() {
    println!("Enter the number of strings:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let num_strings: usize = input.trim().parse()
        .expect("Please enter a valid number");

    let mut strings = Vec::new();
    println!("Enter the strings:");

    for _ in 0..num_strings {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        strings.push(input.trim().to_string());
    }

    let common_prefix = longest_common_prefix(strings);
    println!("Longest common prefix: {}", common_prefix);
}
