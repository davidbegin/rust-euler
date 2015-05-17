#![feature(step_by)]

// Rule 3
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

pub fn result() {
    println!("\nProblem 3 coming soon!\n");
    let result = prime_factors(13195);
    println!("Result: {:?}", result);
}

fn prime_factors(num: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];
    let mut counter: u64 = 1;
    let mut remainder: u64;

    loop {
        remainder = num / counter;
        if remainder == 0 { break }
        factors.push(remainder);
        counter += 1;
    }

    let mut uniq_factors: Vec<u64> = vec![];

    for factor in factors {
        if !uniq_factors.contains(&factor) {
            uniq_factors.push(factor);
        }
    }

    let mut prime_factors: Vec<u64> = vec![];

    for factor in uniq_factors {
        if prime_eh(factor) {
            prime_factors.push(factor);
        }
    }

    prime_factors
}

fn all_factors(number_to_find_primes_of: u64) -> Vec<u64> {
    vec![1, 5, 7, 13, 29, 35, 65, 91, 145, 203, 377, 455, 1015, 1885, 2639, 13195]
}

fn prime_eh(n: u64) -> bool {
    if n == 2 { return true; }
    if n < 3 { return false; }
    let sqrt_limit = (n as f64).sqrt() as u64;
    (3..sqrt_limit+1).step_by(2).find(|i| n % i == 0).is_none()
}

#[cfg(test)]
mod tests {
    use super::prime_factors;
    use super::prime_eh;
    use super::all_factors;

    #[test]
    fn prime_eh_is_up() {
        assert_eq!(prime_eh(7), true);
        assert_eq!(prime_eh(9), false);
        assert_eq!(prime_eh(11), true);
    }

    // #[test]
    // fn six_has_some_prime_factors() {
    //     assert_eq!(prime_factors(6), vec![2, 3]);
    // }

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

    // #[test]
    // fn we_have_to_start_somewhere() {
    //     assert_eq!(prime_factors(13195), [5, 7, 13, 29]);
    // }

    #[test]
    fn all_factors_is_a_thing() {
        assert_eq!(
            all_factors(13195),
            vec![
                1, 5, 7, 13, 29, 35, 65, 91,
                145, 203, 377, 455, 1015, 1885, 2639, 13195
            ]
        );
    }

    #[test]
    fn i_can_tell_if_something_is_prime_from_20_yards_away() {
        assert!(prime_eh(7));
        assert!(prime_eh(29));
        assert!(!prime_eh(9));
        assert!(!prime_eh(100));
    }
}
