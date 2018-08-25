use std::io;

fn main() {
    println!("Enter quantity of fibonacci numbers to generate");
    
    let mut quantity = String::new();
    
    io::stdin().read_line(&mut quantity)
        .expect("Failed to read line");

    let quantity:u32 = quantity.trim().parse()
        .expect("error");

    println!("Generating Fibonacci numbers...\n");
    fibonacci(quantity);
}

fn fibonacci(number: u32) {
    let mut count = 0;
    let mut first_new_value = 0;
    let mut second_new_value = 1;

    if number > 2 {
        count = 2;
        println!("{}", first_new_value);
        println!("{}", second_new_value);
    } else if number == 2 {
        println!("{}", first_new_value);
        return println!("{}", second_new_value);
    } else if number == 1 {
        return println!("{}", first_new_value);
    }

    while count < number {
        let output = first_new_value + second_new_value;
        println!("{}", output);

        first_new_value = second_new_value;
        second_new_value = output;

        count = count + 1;
    };
}