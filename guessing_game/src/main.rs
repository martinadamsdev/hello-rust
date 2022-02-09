use ansi_term::Colour::RGB;
use rand::Rng;
use random_color::{Color, Luminosity, RandomColor};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!(
        "{}",
        RGB(color()[0], color()[1], color()[2]).paint("Guess the number!")
    );

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!(
            "{}",
            RGB(color()[0], color()[1], color()[2]).paint("Please input your guess.")
        );

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn color() -> [u8; 3] {
    return RandomColor::new()
        .hue(Color::Monochrome) // Optional
        .luminosity(Luminosity::Light) // Optional
        .seed(42) // Optional
        .alpha(1.0) // Optional
        .to_rgb_array();
}
