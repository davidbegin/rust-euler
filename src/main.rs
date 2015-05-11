extern crate type_printer;
mod palindrone_checker;
mod product_array_builder;

fn main() {
    let result = product_array_builder::new(2);

    let filtered_result = result
        .iter()
        .filter(|&x| palindrone_checker::call(x) );

    let max = filtered_result.max();
    println!("{:?}", max.unwrap());
}
