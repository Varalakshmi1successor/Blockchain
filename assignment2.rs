use std::io;

fn main() {
    loop {
        println!("Select operation:");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice. Please enter a valid number.");
                continue;
            }
        };

        if choice == 5 {
            println!("Exiting calculator.");
            break;
        }

        println!("Enter first number:");
        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).expect("Failed to read line");
        let num1: f64 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        println!("Enter second number:");
        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).expect("Failed to read line");
        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        match choice {
            1 => perform_operation(&num1, &num2, add),
            2 => perform_operation(&num1, &num2, subtract),
            3 => perform_operation(&num1, &num2, multiply),
            4 => perform_operation(&num1, &num2, divide),
            _ => println!("Invalid choice. Please select a valid operation."),
        }
    }
}

fn perform_operation<F>(num1: &f64, num2: &f64, operation: F)
where
    F: Fn(&f64, &f64) -> f64,
{
    let result = operation(num1, num2);
    println!("Result: {}", result);
}

fn add(x: &f64, y: &f64) -> f64 {
    x + y
}

fn subtract(x: &f64, y: &f64) -> f64 {
    x - y
}

fn multiply(x: &f64, y: &f64) -> f64 {
    x * y
}

fn divide(x: &f64, y: &f64) -> f64 {
    x / y
}
    