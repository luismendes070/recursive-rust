use std::error::Error;

fn factorial(n: u32) -> Result<u32, Box<dyn Error>> {
    if n == 0 {
        Ok(1)
    } else if n == 1 {
        Ok(1)
    }
    else if n <= 1 {
        Err(Box::new(InvalidInputError(n)))
    } else {
        Ok(n * factorial(n - 1)?) // Wrap the expression in Ok
    }
}


#[derive(Debug)]
struct InvalidInputError(u32);

impl std::fmt::Display for InvalidInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid input: Factorial is not defined for {}", self.0)
    }
}

impl Error for InvalidInputError {}

fn main() {
    // Example usage
    match factorial(5) {
        Ok(result) => println!("5! = {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
