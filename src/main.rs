use std::io;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    println!("Fibonacci generator");
    loop {
        println!("Please enter n of the Fibonacci: ");
        let mut fibonacci_number = String::new();
        io::stdin()
        .read_line(&mut fibonacci_number)
        .expect("Failed to read line");

        if fibonacci_number.trim() == "quit" || fibonacci_number.trim() == "stop" {
            break;
        }

        let fibonacci_number: u32 = match fibonacci_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("The Febonnacci of {} is: {}\n\nType 'quit' or 'stop' if you no longer want to continue.\n", fibonacci_number,fibonacci(fibonacci_number))

    }
}