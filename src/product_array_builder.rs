pub fn new(quantity: u32) -> Vec<i32> {
    let mut matrix: Vec<[i32; 2]> = vec![];

    let barrier = largest_number_for_number_of_digits(quantity) as i32;
    for outer in 1..barrier {
        for inner in 1..barrier {
            matrix.push([outer, inner])
        }
    }

    add_dem_pairs_up(matrix)
}

fn add_dem_pairs_up(vec: Vec<[i32; 2]>) -> Vec<i32> {
    let mut final_product_array: Vec<i32> = vec![];

    for arr in vec.iter() {
        final_product_array.push(arr[0]*arr[1]);
    }

    final_product_array
}

pub fn largest_number_for_number_of_digits(number_of_digits: u32) -> u32 {
    let mut sum: u32 = 0;

    for power in 1..number_of_digits + 1 {
        sum += 10u32.pow(power - 1)*9
    }

    sum
}
