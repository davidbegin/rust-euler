#![feature(test)]
extern crate test;
extern crate primal;

// Problem #7

// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13,
// we can see that the 6th prime is 13.
//
// What is the 10 001st prime number?


// So this is pure checking primes
// I will try the primal library first
// for a quick win, and then see what I can do on my own.

pub fn result() {
    println!("\nProblem 7 Coming soon!");

    let result = primal::Primes::all().nth(10001 - 1).unwrap();
    println!("{}", result);
}

pub fn terrible_begin_solution() {
    let mut result = -1;
    let mut primal_count = 0;

    for num in 2..104743 {
        if primal::is_prime(num) {
            primal_count += 1
        }

        if primal_count == 10000 {
            result = num
        }
    }

    println!("Result {}", result);
}


#[cfg(test)]
mod tests {
    use test::Bencher;
    use problem_seven::primal::Primes;

  #[bench]
    fn bench_primal_solution(b: &mut Bencher) {
        b.iter(|| {
            Primes::all().nth(10001 - 1).unwrap();
        });
    }
}
