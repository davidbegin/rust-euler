pub fn call(num: &i32) -> bool {
    let mut master_value = num.clone();
    let mut array_of_digits: Vec<i32> = vec![];

    // turn 8942 => [8, 9, 4, 2]
    loop {
        let last_digit = master_value % 10;
        master_value = master_value / 10;
        array_of_digits.push(last_digit);
        if master_value < 1 { break }
    }

    if array_of_digits.len() % 2 != 0 { return false }

    // check for palindrones
    loop {
        if array_of_digits.len() == 0 { break }
        let a = array_of_digits.pop().unwrap();
        let b = array_of_digits.remove(0);
        if a != b { return false }
    }
    true
}

