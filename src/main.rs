#![feature(test)]
#![feature(step_by)]
#![allow(dead_code, unused_variables, unused_imports, unused_attributes)]

extern crate type_printer;
mod problem_three;
mod problem_four;
mod problem_five;
mod primes_primes_primes;

fn main() {
    primes_primes_primes::whose_got_the_primes();
    // problem_three::result();
    // problem_four::result();
    // problem_five::result();
}
