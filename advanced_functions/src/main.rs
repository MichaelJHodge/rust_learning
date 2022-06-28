fn main() {
    //converts a vector of numbers into a
    //vector of strings using a function
    //as an argument for map.
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
}
