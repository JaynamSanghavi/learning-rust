use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    //immutable variables
    //let _age:i32 = 25;

    //mutable variable
    // let mut var_name : i32 = 25;
    // var_name = 23;

    // println!("Age is {var_name}");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number! ");

    loop {
        println!("Please input your guess: ");

        let mut input_guess: String = String::new();

        // if the input variable is created outside the loop, then it would concat while reading and not override it

        io::stdin()
            .read_line(&mut input_guess)
            .expect("Failed to read user input");

        let input_guess: i32 = match input_guess.trim().parse(){
            Ok(num) => num,
            Err(err) =>  {
                println!("Please enter the number {err}");
                continue;
            }
        };

        match input_guess.cmp(&secret_number) {
            Ordering::Less => println!("Less than secret"),
            Ordering::Equal => {
                println!("Hurrayy!");
                break;
            }
            Ordering::Greater => println!("More than secret"),
        }
    }
}
