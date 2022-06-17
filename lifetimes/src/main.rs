fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
    );
    println!("The longest string is {}", result);
}

use std::fmt::Display;

//This is the longest function from Listing 10-22 that returns 
// the longer of two string slices. But now it has an extra parameter 
// named ann of the generic type T, which can be filled in by any type 
// that implements the Display trait as specified by the where clause. 
// This extra parameter will be printed using {}, which is why the 
// Display trait bound is necessary. Because lifetimes are a type of generic,
//  the declarations of the lifetime parameter 'a and the generic type 
//  parameter T go in the same list inside the angle brackets after the 
//  function name.

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

