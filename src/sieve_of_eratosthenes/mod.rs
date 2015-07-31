extern crate type_printer;
extern crate term;
extern crate primal;
use std::thread;
use std::io::prelude::*;
use std::io;
use std::cmp::Ordering;

pub fn attempt_2() {
    println!("\nSieve Of Eratosthenes");
    println!("=====================\n");

    let mut numbers: Vec<PNumber> = vec![];

    for i in (2..121) {
      let number = PNumber {
        num: i,
        is_prime: true
      };

      numbers.push(number);
    }

    // convert_not_prime_to_prime()
    print_sieve(numbers);
}

#[derive(Debug)]
struct PNumber {
    num: i32,
    is_prime: bool,
}

fn convert_not_prime_to_prime(number_to_convert: PNumber) -> PNumber {
    let num = PNumber {
        num: number_to_convert.num,
        is_prime: true
    };

    num
}

fn print_sieve(numbers: Vec<PNumber>) {
  let mut t = term::stdout().unwrap();

  print!("    ");
  io::stdout().flush().ok().expect("Could not flush stdout");

  for num in numbers {
      if num.is_prime {
        t.fg(term::color::RED).unwrap();
      } else {
        t.fg(term::color::YELLOW).unwrap();
      }

      let spacing: String = if num.num < 10 {
        "   ".to_string()
      } else if num.num < 100 {
        "  ".to_string()
      } else {
        " ".to_string()
      };

      if num.num % 10 == 0 {
        println!("{}", num.num);
      } else {
        print!("{}{}", num.num, spacing);
      }

      io::stdout().flush().ok().expect("Could not flush stdout");

      thread::sleep_ms(30);

      t.reset().unwrap();
  }
}

fn fake_sieve() {
  let mut t = term::stdout().unwrap();

    for i in (2..121) {
      if primal::is_prime(i) {
        t.fg(term::color::RED).unwrap();
      }

      let spacing: String = if i < 10 {
        "   ".to_string()
      } else if i < 100 {
        "  ".to_string()
      } else {
        " ".to_string()
      };

      if i % 10 == 0 {
        println!("{}", i);
      } else {
        print!("{}{}", i, spacing);
      }

      io::stdout().flush().ok().expect("Could not flush stdout");

      thread::sleep_ms(30);
      t.fg(term::color::GREEN).unwrap();
    };

    t.reset().unwrap();
}

pub fn attempt_1() {
    let range = (2..210).map(|i| {
        Number {
            number: i,
            marked: false,
        }
    });

    let (marked_numbers_1, found_primes_1) = sieve(range.collect::<Vec<Number>>(), vec![]);
}

struct Number {
    number: i32,
    marked: bool,
}

fn sieve(sieve_to_filter: Vec<Number>, found_primes: Vec<i32>) -> (Vec<Number>, Vec<i32>) {
    let limit: i32 = 210;
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
            None => {
                let filtered_sieve = sieve_to_filter.iter().map(|i| {
                    if i.marked {
                        Number { marked: true, number: i.number.clone() }
                    } else {
                        Number { marked: false, number: i.number.clone() }
                    }
                });

                return (filtered_sieve.collect::<Vec<_>>() , new_found_primes)
            }
        };

        prime_for_numbers_to_delete_list = prime.clone();
        new_found_primes.push(prime.clone());

        let non_primes_to_delete = (prime_for_numbers_to_delete_list..limit)
            .step_by(prime_for_numbers_to_delete_list)
            .collect::<Vec<i32>>();

        let mut counter: i32 = 1;
        print!(" 0 ");

        let filtered_sieve = sieve_to_filter.iter().map(|i| {
            let should_we_mark_it = match non_primes_to_delete.iter().find(|&x| {
                *x == i.number || i.marked
            }) {
                Some(_) => true,
                None => false
            };

            if counter == 10 {
                println!("\n");
                counter = 1;
            } else {
                counter += 1
            }

            if should_we_mark_it {
                print!(" X ");
                Number { marked: true, number: i.number.clone() }
            } else {
                print!(" O ");
                Number { marked: false, number: i.number.clone() }
            }
        });

        // (filtered_sieve.collect::<Vec<_>>(), new_found_primes)
        sieve(filtered_sieve.collect::<Vec<_>>(), new_found_primes)
    }
}
