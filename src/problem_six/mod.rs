extern crate type_printer;

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
    println!("\n===EXPERIMENTS===\n");
    sum_a_range();
}

fn squares_of_natural_numbers(upper_limit: i32) -> i32 {
    (1i32..upper_limit + 1i32).fold(0, |acc, x| acc + x.pow(2))
}

fn square_of_sum_of_natural_numbers(upper_limit: i32) -> i32 {
  (1..(upper_limit + 1)).fold(0, |acc, x| acc + x).pow(2)
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

// I am continuely engaging is what is what I feel
// an anti-pattern in ruby, and should DEFINITELY be an
// anti-pattern for Rust

// which is setting a mutable variable to 0 or a blank array (vec! doh!)
// and then iterating oversomething mutating it constantly

// I am doing this everywhere and want to learn the rust way


// first example, is the sum of every squared value in a range

// lets start with the first thing we need to be able to do
// sum a range

// attempt to sum a range
fn sum_a_range() {
  let range = 0..10;
  type_printer::print_type_of(&range);

  let result = range.fold(0, |acc, x| acc + x);
  println!("We just folding this: {}", result);

  // println!("Check out this range: {:?}", range);
}

// I would like to benchmark the difference between
// mutating state and and iterating

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        let result = b.iter(|| add_two(2));
        // println!("Result: {:?}", result);
    }
}

