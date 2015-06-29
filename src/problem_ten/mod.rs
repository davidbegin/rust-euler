extern crate primal;

// Problem #10
//
// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//
// Find the sum of all the primes below two million.

pub fn result() {
    println!("\nProblem #10 coming soon\n");
    lets_do_this();
}

// Lets do this the easy way

// first lets iterate from 1 to 2 million

fn lets_do_this() {
  let mut primes: Vec<u64> = vec![];

  for num in 1..2000000u64 {
    if primal::is_prime(num) {
      primes.push(num);
    }
  }

  let result = primes.iter().fold(0, |acc, x| acc + x);
  assert_eq!(result, 142913828922);
  println!("Result: {}", result);
}
