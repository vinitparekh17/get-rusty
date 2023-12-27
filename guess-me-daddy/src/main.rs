use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("---------------------------");
    println!("Welcome to Guess Me Daddy!");
    println!("---------------------------");

    
    // declaring mutable variable type of String
    // the value String::new() will create a new instance of the string type, equivalent to "" (empty string)
    let random_number: u32 = rand::thread_rng().gen_range(1..=50);
    /*
    loop keyword allows to create infinite loop
    */
    loop {
        println!("Enter your guess: ");

        let mut guess = String::new();

        /*
        using io standard lib. to get the input from user via terminal
        .read_line is the method which allows to get a value as string
        passing the variable as a reference by using & sign so method can use guess variable without making copy of it
        &mut used coz reference is also immutable like variable mut keyword allows to use it as mutable
    
        read_line returns Result an enum which can have 2 varients Ok, Err
        Ok  indicates that operation ia successful and contains the result of the operation which in our case will be input string
        Err indicates that operation ia unsuccessful and contains the error msg, a couse of an error
    
        so when Result has Err varient then expect method will take a place
        prints the msg that we passed and will cause our program to crash
        */

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // basic input validation
        // trim it and parse it if ok then run the remaining loop else skip that shit value
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // cmp method which came from standard lib Ordering
        match guess.cmp(&random_number) {
            Ordering::Less => println!("Lmao too small..."),
            Ordering::Greater => println!("OMG, Too big!!"),
            Ordering::Equal => {
                println!("Congrats..");
                break;
            }
        }
    }
}
