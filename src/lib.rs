#![allow(dead_code)]
#![allow(unused_variables)]

// #4
// A palindromic number reads the same both ways.
// The largest palindrome made from the product
// of two 2-digit numbers is 9009 = 91 × 99.
//
// Find the largest palindrome made from the product of two 3-digit numbers.

mod palindrone_checker;

#[test]
fn nine_zero_zero_nine_is_a_palindrone() {
    assert!(palindrone_checker::call(9009));
}

// #[test]
// fn nine_zero_one_nine_is_a_palindrone() {
//     assert!(!palindrone_checker::call(9019));
// }
