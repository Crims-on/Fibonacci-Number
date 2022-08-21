use std::{
    env,
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>>  {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage:");
        println!("fibonacci_number: <number>");
        return Ok(());
    }
    let num: u64 = args[1].trim().parse::<u64>().unwrap();
    let mut fibonacci_3: u128 = 1; // Third number in Fibonacci range
    let mut fibonacci_2: u128 = 1; // Second number in Fibonacci range
    let mut fibonacci_1: u128 = 0; // First number in Fibonacci range
    let mut fibonacci_final: u128 =0; // The lowest number of the bunch is the one requested

    for i in 0..num { //Iterates until it gets to requested number in range
        if fibonacci_3 == fibonacci_2+fibonacci_1 {
            fibonacci_1 = fibonacci_2+fibonacci_3;  
        } 
        else if fibonacci_1 == fibonacci_2+fibonacci_3 {
                fibonacci_2 = fibonacci_3+fibonacci_1;
        }
        else if fibonacci_2 == fibonacci_3+fibonacci_1 {
            fibonacci_3 = fibonacci_1+fibonacci_2;
        }
        }

    if fibonacci_3 >= fibonacci_2 { // Checks for lowest number
        if fibonacci_2 >= fibonacci_1 {
            fibonacci_final = fibonacci_1
        }
        else {fibonacci_final = fibonacci_2}
    } else if fibonacci_3 >= fibonacci_1 {
            fibonacci_final = fibonacci_1
    }
    else {fibonacci_final = fibonacci_3}
    println!("Fibonacci number {num} is {fibonacci_final}");

    Ok(())
}
