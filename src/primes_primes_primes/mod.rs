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


    // example_1();
    exploration_of_marked_num_struct();
}

struct NumberThatCanBeMarked {
    number: i32,
    marked: bool,
}

fn exploration_of_marked_num_struct() {
    let range = (2..121).map(|i| {
        NumberThatCanBeMarked {
            number: i,
            marked: false,
        }
    });

    let lets_see = return_a_sieve_2(range.collect::<_>());

    // let printable_range = range.map(|i| i.marked).collect::<Vec<bool>>();
    // println!("range {:?}", printable_range);
}

fn return_a_sieve_2(vec_to_be_filtered: Vec<NumberThatCanBeMarked>) -> Vec<NumberThatCanBeMarked> {
    let limit: i32 = 121;
    let prime: i32 = vec_to_be_filtered[0].number.clone();

    let non_primes_to_delete = (prime..limit).step_by(prime).collect::<Vec<i32>>();

    let filtered_list = vec_to_be_filtered.iter().map(|i| {
        let should_we_mark_it = match non_primes_to_delete.iter().find(|&x| {
            *x == i.number
        }) {
            Some(_) => false,
            None => true
        };

        if should_we_mark_it {
            NumberThatCanBeMarked { marked: true, number: i.number.clone() }
        } else {
            NumberThatCanBeMarked { marked: false, number: i.number.clone() }
        }
    });

    println!("printing filtered list: {:?}", filtered_list.map(|i| {
        i.marked
    }).collect::<Vec<_>>());

    vec![(NumberThatCanBeMarked { marked: true, number: 2})]
}

fn example_1() {

    // ahh I need to start marking!
    //
    // so I need a vec of an object
    let limit: i32 = 121;
    let range = 2..limit;
    let first_filtered_vec = return_a_sieve(range.collect());
    let second_filtered_vec = return_a_sieve(first_filtered_vec);
    let third_filtered_vec = return_a_sieve(second_filtered_vec);
    let fourth_filtered_vec = return_a_sieve(third_filtered_vec);
    let fifth_filtered_vec = return_a_sieve(fourth_filtered_vec);
    // println!("{:?}", fifth_filtered_vec);

}

fn return_a_sieve(vec_to_be_filtered: Vec<i32>) -> Vec<i32> {
    let limit = 121;

    let prime: i32 = vec_to_be_filtered[0].clone();

    let non_primes_to_delete = (prime..limit).step_by(prime).collect::<Vec<i32>>();

    let filtered_list = vec_to_be_filtered.iter().filter(|i| {
        match non_primes_to_delete.iter().find(|&x| { *x == **i }) {
            Some(_) => false,
            None => true
        }
    });

    filtered_list.map(|i| *i).collect::<Vec<i32>>()
}

fn messy_notes() {
    // Create a list of consecutive integers from 2 through n: (2, 3, 4, ..., n).

    // Initially, let p equal 2, the first prime number.
    // Starting from p, enumerate its multiples by counting to n in increments of p,

    // and mark them in the list (these will be 2p, 3p, 4p, ... ; the p itself should not be marked).

    // Find the first number greater than p in the list that is not marked.
    // If there was no such number, stop. Otherwise,
    // let p now equal this new number (which is the next prime), and repeat from step 3.
}
