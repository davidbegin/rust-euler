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

pub fn terrible_begin_solution() -> usize {
    let mut result: usize = 0;
    let mut primal_count  = 0;
    let mut num: usize    = 1;

    loop {
        if primal::is_prime(num as u64) {
            primal_count += 1;
            if primal_count == 10001 {
                result = num;
                break
            }
        }

        num += 1usize;
    }

    result
}


#[cfg(test)]
mod tests {
    use super::terrible_begin_solution;
    use test::Bencher;
    use problem_seven::primal::Primes;

    #[test]
    fn primal_returns_the_correct_solution() {
        assert_eq!(
            Primes::all().nth(10001 - 1).unwrap(),
            104743
        )
    }

    #[test]
    fn begin_solution_is_the_same_as_primal() {
        assert_eq!(
            Primes::all().nth(10001 - 1).unwrap(),
            terrible_begin_solution()
        )
    }

    #[bench]
    fn bench_primal_solution(b: &mut Bencher) {
        b.iter(|| {
            Primes::all().nth(10001 - 1).unwrap();
        });
    }

    #[bench]
    fn bench_begin_solution(b: &mut Bencher) {
        b.iter(|| {
          terrible_begin_solution();
        });
    }
}
