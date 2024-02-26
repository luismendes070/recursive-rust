use std::error::Error;

fn fibonacci(n: u32) -> Result<u32, Box<dyn Error>> {
    if n <= 1 {
        Ok(n)
    } else {
        let (a, b) = (fibonacci(n - 1)?, fibonacci(n - 2)?);
        Ok(a + b)
    }
}

#[derive(Debug)]
struct InvalidInputError(u32);

impl std::fmt::Display for InvalidInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid input: Fibonacci is not defined for negative numbers.")
    }
}

impl Error for InvalidInputError {}

fn main() {
    // Example usage
    match fibonacci(5) {
        Ok(result) => println!("Fibonacci(5) = {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
