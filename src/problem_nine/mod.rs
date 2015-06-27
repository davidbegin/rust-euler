// Problem #9

// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
//
// a2 + b2 = c2
// For example, 3**2 + 4**2 = 9 + 16 = 25 = 5**2.
//
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

pub fn result() {
    println!("\nProblem #9 coming soon");
    // dang_where_do_i_even_start();
    cogs();
}

fn dang_where_do_i_even_start() {
    // first lets make a array, becuase everything will be the same type and size

    let control: [i32; 3] = [3, 4, 5];

    let power_result = 3i32.pow(2) + 4i32.pow(2);
    assert_eq!(power_result, 25);

    first_two_squared_are_the_last_two_squared();
}

fn first_two_squared_are_the_last_two_squared() {
    let control: [i32; 3] = [3, 4, 5];

    let left_side: i32 = control[0].pow(2) + control[1].pow(2);
    println!("left: {}", left_side);

    let right_side: i32 = control[2].pow(2);
    println!("right: {}", right_side);

}

// How am I going to solve this problem?

// a2 + b2 = c2
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.

// So I could go through every combination that can make 1000,
// and check if a2 + b2 = c2

// So how would I go through every variation of a + b + c = 1000

// how do you try every combination of numbers for 3 digits?


// every combo under 10
// a + b

// like cogs in a machine!

// so a will be the smallest cog

// b medium

// c the largest

// so the lowest any number can be is 998

// so we can iterate to that

// first lets just print out all the combos

fn cogs() {

    // what are good variable names for cogs

    // are their actual terms?

    // Woah this takes way to long
    // how can we optimize
    // for inner in 1..999 {
    //     for middle  in 1..999 {
    //       for outer in 1..999 {
    //         println!("\n{}\n", inner);
    //       }
    //     }
    // }

    // since that takes so long what about the other side

    // a2 + b2 = c2
    // what are all the iterations I need to go through to find these

    // hmmm I'm not sure of the best way to proceed

    // thinking back to the first formula

    // ....or maybe how to find all sets of numbers for pytragreum triangles
    // (yes that is the correct spelling)

    let mut product: i32 = 0;

    for inner in 1..999i32 {
        for middle  in 1..999i32 {
            let outer = 1000i32 - inner - middle;

            if inner.pow(2) + middle.pow(2) == outer.pow(2) {
                product = inner * middle * outer;
            }
        }
    }

    assert_eq!(product, 31875000);
    println!("Product: {}", product);
}
