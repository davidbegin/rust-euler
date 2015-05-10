extern crate type_printer;
mod palindrone_checker;

fn main() {
    println!("\nEuler #4\n");
    // palindrone_checker::call(2084);

    let a = vec![[1, 2], [2, 3], [4, 5]];
    println!("{:?}", a);
   type_printer::print_type_of(&a);
}
