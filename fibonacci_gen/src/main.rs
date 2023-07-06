//Generate the nth Fibonacci number.
//The next number is found by adding up the two numbers before it:
use std::io;

fn main() {
    println!("Which nth Fibonacci number do you want to find?");
    let mut nth: String = String::new();
    io::stdin()
         .read_line(&mut nth)
         .expect("Failed to read line");
    let nth: usize = nth.trim().parse().expect("Please type a number!");

    let mut fib_seq = [0,1];
    for _number in 0..(nth-1) {
        let next: u32 = fib_seq[0] + fib_seq[1];
        fib_seq = [fib_seq[1], next];
        //println!("{}",next) I used this for debugging
    };
    println!("The {nth}th fibonacci number is {}", fib_seq[1]);
}
