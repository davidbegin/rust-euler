#![feature(test)]
#![feature(step_by)]
#![allow(dead_code, unused_variables, unused_imports, unused_attributes)]

extern crate type_printer;
pub mod problem_three;
pub mod problem_four;
pub mod problem_five;
pub mod primes_primes_primes;

fn main() {
    primes_primes_primes::whose_got_the_primes();
    // problem_three::result();
    // problem_four::result();
    // problem_five::result();
}
