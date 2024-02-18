fn main() {
    println!("----- Hello Rust bootcamp 2024! -----");

    // SECTION 1: VARIABLES
    /* let _var_a = "Variable A value";

    let mut _var_b = 1;
    _var_b += 1;

    const _PI: f32 = 3.14;
    let _arr = [1, 2, 3, 4, 5]; */

    // ---------------------------------------------------------
    // SECTION 2: DATA TYPES
    // DATA CASTING
    /* let x: u16 = 300;
    let _y = x as u8;

    let char_1 = 'A';
    let _num_from_char_1 = char_1 as i8;

    let bool_1 = false;
    let _num_from_bool_1 = bool_1 as i8;

    let int_1: u8 = 54;
    let _char_from_int_1 = int_1 as char; */

    // ---------------------------------------------------------
    // SECTION 3: CONTROL FLOW
    // IF ELSE
    /* let if_var = 11;

    if if_var > 10 {
        println!("if_var is greater than 10");
    } else if if_var == 10 {
        println!("if_var is equal 10");
    } else {
        println!("if_var is less than 10");
    } */

    // LOOPING
    // let mut count: i8 = 0;
    /* loop {
        if count > 10 {
            break;
        }
        println!("count is: {count}");
        count += 1;
    } */

    /* while count <= 10 {
        println!("count is: {count}");
        count += 1;
    } */

    /* let mut for_array = [1, 2, 3, 4, 5];
    for number in &mut for_array {
        println!("inital number is: {number}");
        *number += 1;
        println!("updated number is: {number}");
    } */

    /* let mut sum_number = 0;
    for number in 1..11 {
        sum_number += number;
        println!("sum number is: {sum_number}");
    } */

    /* for number in 0..100 {
        if number > 10 {
            break;
        }

        if number % 2 == 0 {
            continue;
        }

        println!("number is: {number}");
    } */

    // ---------------------------------------------------------
    // SECTION 4: FUNCTIONS

    println!("----- End of main func! -----");
}
