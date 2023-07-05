//Convert temperatures between Fahrenheit and Celsius.
use std::io;

fn main() {
    println!("Celsius or Fahrenheit?");
    let mut unit: String = String::new();
    io::stdin()
         .read_line(&mut unit)
         .expect("Failed to read line");
    //println!("{unit}");
    println!("What temperature are you converting?");
    let mut temp: String = String::new(); 
    io::stdin()
    .read_line(&mut temp)
    .expect("Failed to read line");
    let temp: i32 = temp.trim().parse().expect("Please type a number!");
    

    match unit.as_str().trim() { //need to trim off the white space to make it match
        "Celsius" => println!("{temp} {unit} is {} Fahrenheit", c_to_f(temp)),
        "Fahrenheit" => println!("{temp} {unit} is {} Celsius", f_to_c(temp)),
        _ => println!("This isn't in any of the accepted units."), 
    };
}

fn c_to_f(temp: i32) -> i32{
    temp * (9/5) + 32
}

fn f_to_c(temp: i32) -> i32 {
    temp-32 * (5/9)
}