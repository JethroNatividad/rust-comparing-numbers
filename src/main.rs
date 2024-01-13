// program that takes in three numbers, if all numbers are the same, end the program, if not, show the largest number
// Inputs: n1, n2, n3
// Processs: check if equal, get largest number
// Output: The largest number is {}.

fn get_largest_number(vector: Vec<i64>) -> i64 {
    // initialize largest to Minimum value
    // loop vector
    // if number > largest, set largest.
    // return largest
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
}

fn main() {
    println!("Hello, world!");
}
