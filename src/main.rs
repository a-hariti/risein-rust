fn main() {
    crate::task1::main();
    crate::task2::main();
}

mod task1 {
    pub fn concatenate_strings(s1: &str, s2: &str) -> String {
        let mut result = String::new();
        result.push_str(s1);
        result.push_str(s2);
        result
    }
    pub fn main() {
        let string1 = String::from("Hello");
        let string2 = String::from(" World");
        let result = concatenate_strings(&string1, &string2);
        println!("Concatenated string: {}", result);
    }
}

mod task2 {
    use std::{io, str::FromStr};

    #[derive(Debug)]
    pub enum Operation {
        Add,
        Subtract,
        Multiply,
        Divide,
    }

    impl FromStr for Operation {
        type Err = String;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "add" => Ok(Operation::Add),
                "subtract" => Ok(Operation::Subtract),
                "multiply" => Ok(Operation::Multiply),
                "divide" => Ok(Operation::Divide),
                _ => Err(format!("Invalid operation: {}", s)),
            }
        }
    }
    pub fn calculate(op: Option<Operation>, a: f64, b: f64) -> Option<f64> {
        match op {
            Some(Operation::Add) => Some(a + b),
            Some(Operation::Subtract) => Some(a - b),
            Some(Operation::Multiply) => Some(a * b),
            Some(Operation::Divide) => Some(a / b),
            None => None,
        }
    }
    pub fn main() {
        // prompt user for inputs (op, a and b) from stdin
        let mut input = String::new();

        input.clear();
        println!("Enter first number: ");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read stdin");
        let a: f64 = input.trim().parse().expect("failed to parse a");
        // empty the input buffer
        input.clear();

        println!("Enter operation (add, subtract, multiply, divide): ");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read stdin");
        let op = input.trim().parse::<Operation>().ok();
        // empty the input buffer
        input.clear();

        println!("Enter second number: ");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read stdin");
        let b: f64 = input.trim().parse().expect("failed to parse b");
        let result = calculate(op, a, b);
        println!("Result: {:?}", result);
    }
}
