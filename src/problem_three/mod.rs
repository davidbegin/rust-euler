#![feature(step_by)]

// Rule 3
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

pub fn result() {
    println!("\nProblem 3 coming soon!\n");
}

fn is_prime(n: u32) -> bool {
    if n == 2 { return true; }
    if n < 3 { return false; }
    if n % 2 == 0 { return false; }
    let sqrt_limit = ((n as f32).sqrt() as u32);
    (3..sqrt_limit+1).step_by(2).find(|i| n % i == 0).is_none()
}

fn prime_factors(num: u32) -> Vec<u32> {
    let mut factors: Vec<u32> = vec![];

    if num == 0 {
        return factors;
    }

    if num == 2 {
        return vec![2];
    }

    let mut counter: u32 = 1;
    let mut remainder: u32;

    loop {
        if counter == num { break }
        remainder = num % counter;

        if remainder == 0 {
            factors.push(counter);
        } else {
            counter += 1;
            continue
        }

        counter += 1;
    }

    let mut uniq_factors: Vec<u32> = vec![];

    for factor in factors {
        if !uniq_factors.contains(&factor) {
            uniq_factors.push(factor);
        }
    }

    let mut prime_factors: Vec<u32> = vec![];

    for factor in uniq_factors {
        if is_prime(factor) {
            prime_factors.push(factor);
        }
    }

    prime_factors
}

#[cfg(test)]
mod tests {
    use super::prime_factors;
    use super::is_prime;

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
    }

    #[test]
    fn i_can_tell_if_something_is_prime_from_20_yards_away() {
        assert!(is_prime(7));
        assert!(is_prime(29));
        assert!(!is_prime(9));
        assert!(!is_prime(100));
    }
}
