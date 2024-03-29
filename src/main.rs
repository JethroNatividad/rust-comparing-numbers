use std::cmp;
use std::io;
use std::io::Write;

// program that takes in three numbers, if all numbers are the same, end the program, if not, show the largest number
// Inputs: n1, n2, n3
// Processs: check if equal, get largest number
// Output: The largest number is {}.

fn get_largest_number(vector: Vec<i64>) -> i64 {
    // initialize largest to Minimum value
    let mut max: i64 = i64::MIN;
    // loop vector
    // if number > largest, set largest.
    for number in &vector {
        max = cmp::max(*number, max);
    }
    max
}

fn has_equal_elements(vector: Vec<i64>) -> bool {
    vector.iter().all(|&i| i == vector[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_largest_number() {
        assert_eq!(get_largest_number(vec![1, 51, 2]), 51);
        assert_eq!(get_largest_number(vec![9, 1, 32]), 32);
        assert_eq!(get_largest_number(vec![123, 2231, 32121]), 32121);
    }

    #[test]
    fn test_has_equal_elements() {
        assert_eq!(has_equal_elements(vec![1, 51, 2]), false);
        assert_eq!(has_equal_elements(vec![2, 2, 2]), true);
        assert_eq!(has_equal_elements(vec![1, 1, 1, 1, 1, 1, 3]), false);
    }
}

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn main() {
    // initialize an empty vector numbers
    let mut numbers = vec![];
    // loop 3 times
    for i in 1..4 {
        let input: i64 = get_input(&format!("Enter number {}: ", i));
        numbers.push(input);
    }
    // get input and push to vector.
    // if all the same, "All numbers are equal."
    match has_equal_elements(numbers.clone()) {
        true => println!("All numbers are equal."),
        false => {
            // get the largest
            let max: i64 = get_largest_number(numbers.clone());
            // print "The largest number is {}."
            println!("The largest number is {}.", max)
        }
    }
}
