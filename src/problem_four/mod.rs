mod product_array_builder;
mod palindrone_checker;

pub fn result() {
    let result = product_array_builder::new(3);

    let max = result
        .iter()
        .filter(|&x| palindrone_checker::call(x) )
        .max()
        .unwrap();

    println!("{:?}", max);
}
