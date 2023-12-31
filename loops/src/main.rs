/* fn main() {
    loop {
        println!("again!");
    }
} */
/* fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
} */
/* fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10; //since it's in the outer loop, it gets re-declared every loop

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
} */
/* fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
} */
/* fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
} */
/* fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
} */
/* fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
} */
/* fn main() {

    let a = [5; 10]; //note that the semicolon means that this is an array of 5x10s

    let mut sum = 0;

    for x in a {

        sum += x;

    }

    println!("{sum}");

} */
fn main() {

    let mut x = 0;

    'a: loop {

        x += 1;
        println!("{x}");

        'b: loop {

            if x > 10 {

                continue 'a;

            } else {

                break 'b;

            }      

        }

        break;       

    }

}