extern crate type_printer;
extern crate term;
extern crate primal;
use std::thread;
use std::io::prelude::*;
use std::io;
use std::cmp::Ordering;
use std::process::Command;

pub fn attempt_2() {
    print_title();

    let numbers = starting_sieve();
    let mut prime_increamentor = 0;
    sieve_cycle(&numbers, prime_increamentor);
}

fn sieve_cycle(sieve: &Vec<PNumber>, old_prime_increamentor: i32) {
  let new_prime_increamentor = find_next_non_prime_number(
      &sieve, &old_prime_increamentor
  );

  let mut next_non_prime_number = new_prime_increamentor + new_prime_increamentor;

  let result: Vec<PNumber> = sieve
      .iter()
      .map(|pnum| {

    if pnum.num == next_non_prime_number {
      next_non_prime_number += new_prime_increamentor;
      convert_prime_to_not_prime(pnum)
    } else {
      PNumber {
        num: pnum.num,
        is_prime: pnum.is_prime
      }
    }

  }).collect::<Vec<PNumber>>();

  print_title();
  println!("\nOld Prime Incrementor: {}", old_prime_increamentor);
  println!("New Prime Incrementor: {}\n", new_prime_increamentor);

  print_sieve(&result);
  if new_prime_increamentor == 113 {
    return;
  } else {
    sieve_cycle(&result, new_prime_increamentor);
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
}

fn print_title() {
    clear_screen();
    println!("\nSieve Of Eratosthenes");
    println!("=====================\n");
    print_legend();
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

fn print_legend() {
  let mut t = term::stdout().unwrap();
  io::stdout().flush().ok().expect("Could not flush stdout");

  t.fg(term::color::BRIGHT_MAGENTA).unwrap();
  println!("PRIME");
  t.fg(term::color::BRIGHT_YELLOW).unwrap();
  println!("NOT PRIME");

  io::stdout().flush().ok().expect("Could not flush stdout");
}

fn print_sieve(numbers: &Vec<PNumber>) {
  let mut t = term::stdout().unwrap();

  print!("    ");
  io::stdout().flush().ok().expect("Could not flush stdout");

  for num in numbers {
      if num.is_prime {
        t.fg(term::color::BRIGHT_MAGENTA).unwrap();
      } else {
        t.fg(term::color::BRIGHT_YELLOW).unwrap();
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
      thread::sleep_ms(5);

      t.reset().unwrap();
  }
}
