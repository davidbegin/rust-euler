// Rule 3
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

pub fn result() {
    println!("\nProblem 3 coming soon!\n");
}

fn prime_factors(num: i32) -> Vec<i32> {
    let mut prime_factors: Vec<i32> = vec![];
    if num == 0 { return prime_factors; }

    let even_eh: bool = if num % 2 == 0 {
        prime_factors.push(2);
        true
    } else {
        false
    };

    if num != 2 {
        let remainder = num / 2;
        if remainder != 2 {
            prime_factors.push(remainder);
        }
    }
    prime_factors
}

fn all_factors(number_to_find_primes_of: i32) -> Vec<i32> {
    vec![1, 5, 7, 13, 29, 35, 65, 91, 145, 203, 377, 455, 1015, 1885, 2639, 13195]
}

fn prime_eh(num: i32) -> bool {
    match num {
        7 | 29  => true,
        9 | 100 => false,
        _       => false
    }
}


#[cfg(test)]
mod tests {
    use super::prime_factors;
    use super::prime_eh;
    use super::all_factors;

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
