extern crate type_printer;
extern crate term;
extern crate primal;
use std::thread;
use std::io::prelude::*;
use std::io;
use std::cmp::Ordering;
use std::process::Command;

pub fn attempt_2() {
    clear_screen();


    let numbers = starting_sieve();
    print_sieve(&numbers);

    let mut prime_increamentor = 0;

    loop {
      prime_increamentor = find_next_non_prime_number(&numbers, &prime_increamentor);
      let mut next_non_prime_number = prime_increamentor + prime_increamentor;

      let result: Vec<PNumber> = numbers.iter().map(|pnum| {
          if pnum.num == prime_increamentor {
            convert_prime_to_not_prime(pnum)
          } else if pnum.num == next_non_prime_number {
            next_non_prime_number += prime_increamentor;
            convert_prime_to_not_prime(pnum)
          } else {
            PNumber {
              num: pnum.num,
              is_prime: pnum.is_prime
            }
          }
      }).collect::<Vec<PNumber>>();

      clear_screen();
      print_sieve(&result);
    }
}

fn starting_sieve() -> Vec<PNumber> {
    let mut numbers: Vec<PNumber> = vec![];

    for i in (2..121) {
      let number = PNumber {
        num: i,
        is_prime: true
      };

      numbers.push(number);
    }
    numbers
}

fn find_next_non_prime_number(numbers: &Vec<PNumber>, prime_increamentor: &i32) -> i32 {
    let next_non_prime_number = numbers.iter().find(|pnum| {
      primal::is_prime(pnum.num as u64) && pnum.num > prime_increamentor.clone()
    });

    next_non_prime_number.unwrap().num
}

fn clear_screen() {
    let output = Command::new("clear").output().unwrap_or_else(|e| {
      panic!("failed to execute process: {}", e)
    });

    println!("{}", String::from_utf8_lossy(&output.stdout));
    print_title();
}

fn print_title() {
    println!("\nSieve Of Eratosthenes");
    println!("=====================\n");
}

#[derive(Debug)]
struct PNumber {
    num: i32,
    is_prime: bool,
}

fn convert_prime_to_not_prime(number_to_convert: &PNumber) -> PNumber {
    let num = PNumber {
        num: number_to_convert.num,
        is_prime: false
    };

    num
}

// I need to convert this to not take ownership
fn print_sieve(numbers: &Vec<PNumber>) {
  let mut t = term::stdout().unwrap();

  print!("    ");
  io::stdout().flush().ok().expect("Could not flush stdout");

  for num in numbers {
      if num.is_prime {
        t.fg(term::color::BRIGHT_MAGENTA).unwrap();
      } else {
        t.fg(term::color::WHITE).unwrap();
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

      // Make constant
      thread::sleep_ms(20);

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
