pub fn new(quantity: i32) -> Vec<[i32; 2]> {
    let mut product_array: Vec<[i32; 2]> = vec![];
    for outer in 1..99 {
        for inner in 1..99 {
            product_array.push([outer, inner])
        }
    }

    product_array
}
