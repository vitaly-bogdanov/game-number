use std::{
    io,
    cmp::Ordering,
};
use rand::Rng;

fn main() {
    let secret_number: i64 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please, input number: ");
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("error read line");
        let guess: i64 = guess.trim().parse().expect("invalid number");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Equal!");
                break;
            },
            Ordering::Greater => println!("Greater"),
            Ordering::Less => println!("Less")
        };
    }
}
