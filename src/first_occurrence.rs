use std::io;

fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &num) in arr.iter().enumerate() {
        if num == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    println!("Enter the size of the array:");
    let mut size_input = String::new();
    io::stdin().read_line(&mut size_input)
        .expect("Failed to read line");
    let _size: usize = size_input.trim().parse()
        .expect("Please enter a valid number");

    println!("Enter the elements of the array separated by space:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let arr: Vec<i32> = input.trim().split_whitespace()
        .map(|num| num.parse().expect("Please enter valid numbers"))
        .collect();

    println!("Enter the number to search:");
    let mut target_input = String::new();
    io::stdin().read_line(&mut target_input)
        .expect("Failed to read line");
    let target: i32 = target_input.trim().parse()
        .expect("Please enter a valid number");

    match find_first_occurrence(&arr, target) {
        Some(index) => println!("The first occurrence of {} is at index {}.", target, index),
        None => println!("The number {} is not found in the array.", target),
    }
}
