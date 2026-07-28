use crate::{christmas_song::print_lyrics, fahrenheit_to_celsius::ftoc, fibonacci::fibonacci};

mod christmas_song;
mod fahrenheit_to_celsius;
mod fibonacci;

fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    println!("451F in Celsius: {}", ftoc(451.0));
    println!("6th Fibonacci number is: {}", fibonacci(6));
    println!("Crsitmas song:");
    print_lyrics();
}
