// The sum of the squares of the first ten natural numbers is,
//
// 1**2 + 2**2 + ... + 10**2 = 385
// The square of the sum of the first ten natural numbers is,
//
// (1 + 2 + ... + 10)**2 = 552 = 3025
// Hence the difference between the sum of the squares
// of the first ten natural numbers and the square
// of the sum is 3025 − 385 = 2640.
//
// Find the difference between the sum of the squares
// of the first one hundred natural numbers and
// the square of the sum.

pub fn result() {
    let result = difference(100);
    println!("\nThe to answer to problem 6 is: {}", result);
}

fn squares_of_natural_numbers(upper_limit: i32) -> i32 {
    let mut result: i32 = 0;

    for num in (1i32..upper_limit + 1i32) {
        result += num.pow(2);
    }

    result
}

fn square_of_sum_of_natural_numbers(upper_limit: i32) -> i32 {
    let mut result: i32 = 0;

    for num in 1..(upper_limit + 1) {
        result += num
    }

    result.pow(2)
}

fn difference(upper_limit: i32) -> i32 {
    square_of_sum_of_natural_numbers(upper_limit)
        - squares_of_natural_numbers(upper_limit)
}

#[test]
fn test_squares_of_natural_numbers() {
    assert_eq!(squares_of_natural_numbers(10), 385);
}

#[test]
fn test_square_of_sum_of_natural_numbers() {
    assert_eq!(square_of_sum_of_natural_numbers(10), 3025);
}

#[test]
fn test_difference_in_sum_squaring() {
    assert_eq!(difference(10), 2640);
}
