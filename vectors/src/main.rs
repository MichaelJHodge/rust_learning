use std::collections::HashMap;



fn main() {
    let mut numbers = vec![1, 2, 3, 4, 4, 5, 6, 7, 8, 1000, 20, 9, 10, 20, 50, 50, 75, 75, 75];
    
    if numbers.is_empty() {
        panic!("No numbers!")
    } else {
        println!("The list of ints are: {:?}", numbers);
        println!("Mean: {}", calculate_mean(&numbers));
        println!("Mode: {:?}", calculate_mode(&numbers));
        println!("Median: {}", calculate_median(&mut numbers));
    }
    


}

fn calculate_mean(numbers: &Vec<i32>) -> f32 {

    //Iterates through the vector and calculates the sum. 
    let sum: i32 = numbers.iter().sum();

    sum as f32 / numbers.len() as f32
}

fn calculate_mode(numbers:  &Vec<i32>) -> Vec<i32>  {
    //Can allocate the size of the map since we know the size
    //of the vector - important since this is all stored
    //on the heap.

    let mut numbers_map = HashMap::with_capacity(numbers.len());

    //iterates over the numbers vector and counts how many times
    //each number appears in the vector. It increments the number
    //to keep track of it. The first time we see a number, we first insert
    //the value 0. The numbers are keys and the count is the value we 
    //keep track of.

    for number in numbers {
        let count = numbers_map.entry(number).or_insert(0);
        *count += 1;
    }

    let max = numbers_map.values().cloned().max().unwrap_or(0);

    numbers_map.into_iter()
    .filter(|&(_, v)| v == max)
    .map(|(&k, _)| k)
    .collect()
     



}

//Pass in a mutable reference to keep with the ownership rules

fn calculate_median(numbers: &mut Vec<i32>) -> i32 {

    //Sort the vector in case values are out of order
    numbers.sort();

    //median is the length of the vector divided by 2 to get
    //the halfway point
    let median = numbers.len() / 2;

    //checks if the length of the vector divided by 2 has a 
    //remainder of 0. If the remainder is 0, it just means the 
    // # is even, and it uses the calculate_mean function.
    
    if numbers.len() % 2 == 0 {
        calculate_mean(&vec![numbers[median - 1], numbers[median]]) as i32

    } else {
        numbers[median]
    }
}