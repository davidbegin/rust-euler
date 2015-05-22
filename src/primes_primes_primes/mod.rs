extern crate type_printer;

pub fn whose_got_the_primes() {
    println!("\n\tI need to learn more about primes.");
    println!("\n\t...and numbers in general, as my math skills are severly lacking.");
    println!("\n\t-----------------------------------------------------------------");

    println!("\t
        Things I want to learn:

        different ways of checking primality and how to implement them in rust

        also I want to make the sieve of eratosthenes thing
        ...it just sounds cool
    ");

    // First let me have fun with sieve of eratosthenes thang
    //
    // I'd also like to maybe animate it,
    // although that's probably beyond my skills



    // Create a list of consecutive integers from 2 through n: (2, 3, 4, ..., n).

    let n: i32 = 121;
    let range = 2..n;

    // Initially, let p equal 2, the first prime number.

    let p: i32 = 2;

    // Starting from p, enumerate its multiples by counting to n in increments of p,
    let ps_to_delete = (p..n).step_by(p).collect::<Vec<i32>>();

    let filtered_list = range.filter(|i| {
        match ps_to_delete.iter().find(|&x| { *x == *i }) {
            Some(_) => false,
            None => true
        }
    });

    println!("filtered_list: {:?}", filtered_list.collect::<Vec<_>>());
    // now I need to make a filtered vec

    // and mark them in the list (these will be 2p, 3p, 4p, ... ; the p itself should not be marked).

    // Find the first number greater than p in the list that is not marked.
    // If there was no such number, stop. Otherwise,
    // let p now equal this new number (which is the next prime), and repeat from step 3.
}
