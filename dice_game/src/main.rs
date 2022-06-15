use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn main() {
    println!("Begin! Player 1, roll your dice!");

    let dice_faces = rand::thread_rng().gen_range(1..6);

    loop {
        println!("Press enter to roll");

        let mut dice_roll = String::new();

        io::stdin()
            .read_line(&mut dice_roll)
            .expect("Failed to read line");

        let dice_roll: u32 = match dice_roll.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You rolled a: {}", dice_roll);

        match dice_roll.cmp(&dice_faces) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}