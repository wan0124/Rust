use std::io;
use std::cmp::Ordering;
use rand::Rng;
//standard library -> io library
fn main() {

    println!("Guess the number");

    //start..=end inclusive lower bound and upper bound
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");
    
    loop{
        println!("Please input your guess.");

        // in Rust, variable is immutable by default, add mut to make it mutable
        //"::" means new is an associated function of the String type
        let mut guess = String::new();// new empty string

        io::stdin()
            .read_line(&mut guess)//also need mut here
            .expect("Failed to read line");

        /*
        let x = 5;
        let y = 10;
    
        println!("x = {x} and y + 2 = {}", y+2);
        
        {x} -> print variable x
        {} -> find the following expressions in the same order
    
        result : x = 5 and y + 2 = 12
        */
    
        println!("your guessed : {guess}");
        /*
        Because you want to use cmp to compare two integer, but the origin type
        of guess is String, so at first you should parse guess to u32
        (unsigned 32-bit).
        The parse function will return Result just like read_line does, so you
        need an expect.
        */
        //let guess: u32 = guess.trim().parse().expect("Please type a number");

        /*
        Because the parse function return a Result, so you can use match to choose
        the diffrent way dealing with Ok or Err.
        Ok means parse success, and return the num. So if the Result matches Ok, than
        we return num to aply to guess.
        Err means parse error, and the _ means that catchall value. wW’re saying we 
        want to match all Err values, no matter what information they have inside them.
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
