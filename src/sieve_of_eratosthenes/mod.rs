extern crate type_printer;
pub fn attempt_1() {
    println!("\nSieve Of Eratosthenes");
    println!("=====================\n");

    let range = (2..121).map(|i| {
        Number {
            number: i,
            marked: false,
        }
    });

    let (marked_numbers, found_primes) = sieve(range.collect::<Vec<Number>>(), vec![]);

    println!("found primes for iter 1: {:?}", found_primes);
}

struct Number {
    number: i32,
    marked: bool,
}

fn sieve(sieve_to_filter: Vec<Number>, found_primes: Vec<i32>) -> (Vec<Number>, Vec<i32>) {
    let limit: i32 = 121;
    let mut new_found_primes: Vec<i32> = found_primes.clone();
    let mut prime_for_numbers_to_delete_list: i32;

    {
        let prime_option = sieve_to_filter.iter().find(|i| {
            let is_already_in_the_prime_list = match found_primes
                .iter()
                .find(|fp| **fp == i.number ) {
                    Some(_) => true,
                    None => false
                };
            !i.marked && !is_already_in_the_prime_list
        });

        let prime = match prime_option {
            Some(x) => x.number,
            None => 2,
        };

        prime_for_numbers_to_delete_list = prime.clone();
        new_found_primes.push(prime.clone());
    }

    let non_primes_to_delete = (prime_for_numbers_to_delete_list..limit)
        .step_by(prime_for_numbers_to_delete_list)
        .collect::<Vec<i32>>();

    let filtered_sieve = sieve_to_filter.iter().map(|i| {
        let should_we_mark_it = match non_primes_to_delete.iter().find(|&x| {
            *x == i.number
        }) {
            Some(_) => false,
            None => true
        };

        if should_we_mark_it {
            Number { marked: true, number: i.number.clone() }
        } else {
            Number { marked: false, number: i.number.clone() }
        }
    });

    (filtered_sieve.collect::<Vec<_>>() , new_found_primes)
}



    // let should_we_continue = match prime_option {
    //     Some(_) => true,
    //     None => false,
    // };
    //
    // if !should_we_continue {
    //     println!("\n\t
    //          ...breaking because we did not find any unmarked number
    //          that wasn't already in the list of primes\n
    //     ");
    //
    //     // return (vec![Number { number: 2, marked: false }], vec![2])
    //     return (vec![Number { number: 2, marked: false }], new_found_primes);
    //     // return (sieve_to_filter, new_found_primes);
    // }
