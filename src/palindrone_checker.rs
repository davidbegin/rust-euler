#![allow(unused_variables)]
pub fn call(num: i32) -> bool {
    let mut master_value = num.clone();
    let mut array_of_digits: Vec<i32> = vec![];

    loop {
        let last_digit = master_value % 10;
        master_value = master_value / 10;
        array_of_digits.push(last_digit);
        if master_value < 1 { break }
    }

    println!("array of digits: {:?}", array_of_digits);
    println!("length {}", array_of_digits.len());

    let odd_eh = array_of_digits.len() % 2 != 0;

    if odd_eh {
        return false
    }

    loop {
        if array_of_digits.len() == 0 { break }
        let a = array_of_digits.pop().unwrap();
        let b = array_of_digits.remove(0);
        println!("a {:?}", a);
        println!("b {:?}", b);

        if a != b { return false }
    }

    true
}

