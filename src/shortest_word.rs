use std::io;

fn shortest_word_in_string(s: &str) -> Option<&str> {
    let shortest_word = s.split_whitespace().min_by_key(|word| word.len());
    shortest_word
}

fn main() {
    println!("Enter a string of words:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    if let Some(shortest_word) = shortest_word_in_string(&input) {
        println!("The shortest word in the string is: {}", shortest_word);
    } else {
        println!("No words found in the string.");
    }
}