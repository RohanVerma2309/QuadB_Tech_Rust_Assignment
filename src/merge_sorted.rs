use std::io;

fn merge_sorted_arrays(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }
    while i < arr1.len() {
        result.push(arr1[i]);
        i += 1;
    }
    while j < arr2.len() {
        result.push(arr2[j]);
        j += 1;
    }

    result
}

fn main() {
    println!("Enter the size of the first array:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let _size1: usize = input.trim().parse()
        .expect("Please enter a valid number");

    println!("Enter the elements of the first array separated by space:");
    input.clear();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let arr1: Vec<i32> = input.trim().split_whitespace()
        .map(|num| num.parse().expect("Please enter valid integers"))
        .collect();

    println!("Enter the size of the second array:");
    input.clear();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let _size2: usize = input.trim().parse()
        .expect("Please enter a valid number");

    println!("Enter the elements of the second array separated by space:");
    input.clear();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let arr2: Vec<i32> = input.trim().split_whitespace()
        .map(|num| num.parse().expect("Please enter valid integers"))
        .collect();

    let merged_array = merge_sorted_arrays(arr1, arr2);
    println!("Merged and sorted array: {:?}", merged_array);
}
