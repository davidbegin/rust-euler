pub fn new(_quantity: i32) -> Vec<i32> {
    let mut matrix: Vec<[i32; 2]> = vec![];

    for outer in 1..99 {
        for inner in 1..99 {
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
