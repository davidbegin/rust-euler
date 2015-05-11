#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate type_printer;
mod palindrone_checker;
mod product_array_builder;

fn main() {
    problem_four();
}

fn problem_four() {
    let result = product_array_builder::new(3);

    let max = result
        .iter()
        .filter(|&x| palindrone_checker::call(x) )
        .max()
        .unwrap();

    println!("{:?}", max);
}
