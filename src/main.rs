use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Input a number and this program will give back your Collatz!"); 

    let mut input = String::new(); 
    let mut collatz_value = 1;
    const TERMINATION_POINT: u128 = 1;

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut input: u128 = match input.trim().parse() { 
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    loop {
        match input.cmp(&TERMINATION_POINT) { 
            Ordering::Less => { 
                println!("ERROR!");
                break;
            }
            
            Ordering::Greater => { 
                if input % 2 == 0 { 
                    input = shrink(input);
                    collatz_value += 1;
                }
                else { 
                    input = grow(input);
                    collatz_value += 2; 
                }
            }

            Ordering::Equal => {
                println!("collatz_value = {collatz_value}");
                break;
            }
        }
    }
}

fn shrink(value: u128) -> u128 { 
    return value/2; 
}

fn grow(value: u128) -> u128 { 
    return (value*3 + 1)/2;
}

