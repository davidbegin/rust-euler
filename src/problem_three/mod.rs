// Rule 3
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

#![feature(test)]
extern crate test;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn result() {
    println!("\nProblem 3 coming soon!\n");
}

fn is_prime(n: u64) -> bool {
    if n == 2 { return true; }
    if n < 3 { return false; }
    if n % 2 == 0 { return false; }
    let sqrt_limit = (n as f64).sqrt() as u64;
    (3..sqrt_limit+1).step_by(2).find(|i| n % i == 0).is_none()
}

fn prime_factors(num: u64) -> Vec<u64> {
    if num == 0 { return vec![]; }
    if num == 2 { return vec![2]; }

    find_factors(num).iter()
        .filter(|&i| is_prime(*i))
        .map(|i| *i)
        .collect::<Vec<u64>>()
}

fn find_factors(num: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];
    let mut counter: u64 = 1;

    loop {
        if counter == num { break }
        let remainder: u64 = num % counter;
        if remainder == 0 {
            factors.push(counter);
        } else {
            counter += 1;
            continue
        }
        counter += 1;
    }

    factors
}

fn largest_prime_factor(num: u64) -> u64 {
    let mut factors = prime_factors(num);
    let result = factors.pop();
    result.unwrap()
}

#[cfg(test)]
mod tests {
    use super::prime_factors;
    use super::largest_prime_factor;
    use super::is_prime;
    use problem_three::test::Bencher;
    use super::*;

    #[bench]
    fn bench_prime_factors(b: &mut Bencher) {
        b.iter(|| {
            prime_factors(6)
        });
    }

    #[test]
    fn prime_eh_is_up() {
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(11), true);
    }

    #[test]
    fn six_has_some_prime_factors() {
        assert_eq!(prime_factors(6), vec![2, 3]);
    }

    #[test]
    fn two_is_only_prime_factor_of_two() {
        assert_eq!(prime_factors(4), vec![2]);
    }

    #[test]
    fn two_is_the_only_prime_factor_of_two() {
        assert_eq!(prime_factors(2), vec![2]);
    }

    #[test]
    fn there_are_no_prime_factors_of_zero() {
        assert_eq!(prime_factors(0), vec![]);
    }

    #[test]
    fn we_have_to_start_somewhere() {
        assert_eq!(prime_factors(13195), vec![5, 7, 13, 29]);
        assert_eq!(prime_factors(494994), vec![2, 3, 82499]);
        assert_eq!(prime_factors(494994111), vec![3, 131, 1259527]);
        // assert_eq!(prime_factors(600851475143), vec![2, 3, 82499]);
    }

    #[test]
    fn i_can_tell_if_something_is_prime_from_20_yards_away() {
        assert!(is_prime(7));
        assert!(is_prime(29));
        assert!(!is_prime(9));
        assert!(!is_prime(100));
    }

    // #[test]
    // fn what_the_heck() {
    //     assert_eq!(prime_factors(600851475143), vec![71, 839, 1471, 6857]);
    // }

    // #[test]
    // fn i_can_actually_solve_euler_3() {
    //     assert_eq!(largest_prime_factor(600851475143), 6857);
    // }

}
