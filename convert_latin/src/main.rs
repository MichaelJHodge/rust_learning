

const CONSONANTS: [char; 21] = [
    'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'x', 'z',
    'w', 'y',
];

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];




fn main() {
    print_converted("Michael");
    print_converted("Hodge");
}


fn convert_enlish(s: &str) -> String {

    let first_char = match s.chars().next() {
        Some(first_char) => first_char,
        None => return String::new(),
    };

    if CONSONANTS.contains(&first_char.to_ascii_lowercase()) {
        s.chars()
            .skip(1)
            .chain(format!("-{}ay", first_char).chars())
            .collect::<String>()
    } else if VOWELS.contains(&first_char.to_ascii_lowercase()) {
        format!("{}-hay", s)
    } else {
        String::from(s)
    }

}

fn print_converted(s: &str) {
    println!("{} -> {}", s, convert_enlish(s));

}


  