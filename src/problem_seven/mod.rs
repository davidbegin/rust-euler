// Problem #7

// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13,
// we can see that the 6th prime is 13.
//
// What is the 10 001st prime number?


// So this is pure checking primes
// I will try the primal library first
// for a quick win, and then see what I can do on my own.

extern crate primal;

pub fn result() {
    println!("\nProblem 7 Coming soon!");

    let result = primal::Primes::all().nth(10001 - 1).unwrap();

    println!("{}", result);
}
