// Problem # 5
// 2520 is the smallest number that can be divided
// by each of the numbers from 1 to 10 without any remainder.
//
// What is the smallest positive number that is evenly divisible
// by all of the numbers from 1 to 20?

pub fn result() {
    println!("\nProblem 5 coming soon!\n");

    let num = 2520;
    assert_eq!(smallest_num_finder(), 2520);
}

fn smallest_num_finder() -> i32 {
    let mut result: i32 = 0;

    for x in 1..100000 {
        if divisible_by_all_numbers(x) {
            result = x;
            break;
        }
    }

    result
}

fn divisible_by_all_numbers(num: i32) -> bool {
    let mut divisible_by_all_numbers_eh: bool = true;

    for x in 1..11 {
        if num % x != 0 {
            divisible_by_all_numbers_eh = false;
        }
    }

    divisible_by_all_numbers_eh
}
