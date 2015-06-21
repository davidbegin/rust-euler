// Rule 3
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

#![feature(test)]
extern crate test;
extern crate primal;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn result() {
    println!("\nProblem 3 coming soon!\n");
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

fn find_factors_2(num: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];

    for counter in 1..num {
        let remainder: u64 = num % counter;
        if remainder == 0 {
            factors.push(counter);
        } else {
            continue
        }
    }

    factors
}

fn find_factors_3(num: u64) -> Vec<u64> {
    (1..num).filter(|&i|  num % i == 0).collect::<Vec<_>>()
}

fn largest_prime_factor(num: u64) -> u64 {
    let mut factors = prime_factors(num);
    let result = factors.pop();
    result.unwrap()
}

fn is_prime(n: u64) -> bool {
    if n == 2 { return true; }
    if n < 3 { return false; }
    if n % 2 == 0 { return false; }
    let sqrt_limit = (n as f64).sqrt() as u64;
    (3..sqrt_limit+1).step_by(2).find(|i| n % i == 0).is_none()
}

fn is_prime_2(n: u64) -> bool {
  primal::is_prime(n)
}

fn prime_factors(num: u64) -> Vec<u64> {
    if num == 0 { return vec![]; }
    if num == 2 { return vec![2]; }

    find_factors(num).iter()
        .filter(|&i| is_prime(*i))
        .map(|i| *i)
        .collect::<Vec<u64>>()
}

fn prime_factors_1(num: u64) -> Vec<u64> {
    if num == 0 { return vec![]; }
    if num == 2 { return vec![2]; }

    find_factors(num).iter()
        .filter(|&i| is_prime(*i))
        .map(|&i| i)
        .collect::<Vec<u64>>()
}

fn prime_factors_2(num: u64) -> Vec<u64> {
    if num == 0 { return vec![]; }
    if num == 2 { return vec![2]; }

    find_factors_2(num).iter()
        .filter(|&i| is_prime(*i))
        .map(|&i| i)
        .collect::<Vec<u64>>()
}

fn prime_factors_3(num: u64) -> Vec<u64> {
    if num == 0 { return vec![]; }
    if num == 2 { return vec![2]; }

    find_factors_3(num).iter()
        .filter(|&i| is_prime(*i))
        .map(|&i| i)
        .collect::<Vec<u64>>()
}

fn prime_factors_4(num: u64) -> Vec<u64> {
    if num == 0 { return vec![]; }
    if num == 2 { return vec![2]; }

    find_factors_3(num).iter()
        .filter(|&i| is_prime_2(*i))
        .map(|&i| i)
        .collect::<Vec<u64>>()
}

fn everything_in_one_kind_of(num: u64) -> Vec<u64> {
    if num == 0 { return vec![]; }
    if num == 2 { return vec![2]; }

    (2..num).filter(|i| {
        is_prime_factor(num, i)
    }).collect::<Vec<_>>()
}

fn is_prime_factor(num: u64, index: &u64) -> bool {
    if num % 2 == 0 { return false; }
    if num % index != 0 { return false; }
    if num < 3 { return false; }
    true

    // let sqrt_limit = (num as f64).sqrt() as u64;
    // (3..sqrt_limit+1).step_by(2).find(|i| num % i == 0).is_none()
}

#[cfg(test)]
mod tests {
    use super::everything_in_one_kind_of;
    use super::prime_factors;
    use super::prime_factors_1;
    use super::prime_factors_2;
    use super::prime_factors_4;
    use super::find_factors;
    use super::find_factors_2;
    use super::find_factors_3;
    use super::largest_prime_factor;
    use super::is_prime;
    use super::is_prime_2;
    use problem_three::test::Bencher;
    use super::*;

    #[bench]
    fn bench_everything_in_one_kind_of_method(b: &mut Bencher) {
        b.iter(|| {
            everything_in_one_kind_of(999)
        });
    }

    #[bench]
    fn bench_prime_find_factors(b: &mut Bencher) {
        b.iter(|| {
            find_factors(999)
        });
    }

    #[bench]
    fn bench_prime_find_factors_2(b: &mut Bencher) {
        b.iter(|| {
            find_factors_2(999)
        });
    }

    #[bench]
    fn bench_prime_find_factors_3(b: &mut Bencher) {
        b.iter(|| {
            find_factors_3(999)
        });
    }

    #[bench]
    fn bench_prime_factors_4(b: &mut Bencher) {
        b.iter(|| {
            prime_factors_4(999)
        });
    }

    #[bench]
    fn bench_prime_factors_2(b: &mut Bencher) {
        b.iter(|| {
            prime_factors_2(12)
        });
    }

    #[bench]
    fn bench_prime_factors_optimization_attempt_1(b: &mut Bencher) {
        b.iter(|| {
            prime_factors_1(12)
        });
    }

    #[bench]
    fn bench_prime_factors(b: &mut Bencher) {
        b.iter(|| {
            prime_factors(6)
        });
    }

    #[bench]
    fn bench_is_prime(b: &mut Bencher) {
      b.iter(|| {
        is_prime(3557)
      });
    }

    #[bench]
    fn bench_is_prime_2(b: &mut Bencher) {
      b.iter(|| {
        is_prime_2(3557)
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
        // assert_eq!(prime_factors(494994111), vec![3, 131, 1259527]);
        // assert_eq!(prime_factors_4(600851475143), vec![2, 3, 82499]);
    }

    #[test]
    fn i_can_tell_if_something_is_prime_from_20_yards_away() {
        assert!(is_prime(7));
        assert!(is_prime(29));
        assert!(!is_prime(9));
        assert!(!is_prime(100));
    }

    #[test]
    fn what_the_heck() {
        // assert_eq!(prime_factors(600851475143), vec![71, 839, 1471, 6857]);
        // assert_eq!(everything_in_one_kind_of(600851475143), vec![71, 839, 1471, 6857]);
        // assert_eq!(everything_in_one_kind_of(1475143), vec![29, 50867]);
    }

    // #[test]
    // fn i_can_actually_solve_euler_3() {
    //     assert_eq!(largest_prime_factor(600851475143), 6857);
    // }

}
