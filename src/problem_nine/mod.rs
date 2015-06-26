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
    dang_where_do_i_even_start();
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
