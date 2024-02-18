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
    // SECTION 4: FUNCTIONS & CLOSURES
    // FUNCTIONS
    /* fn hello_func(first_name: &str, last_name: &str) {
        println!("Hello {first_name} {last_name}!")
    }

    hello_func("Duc", "Huynh"); */

    /* fn add_func(array: &[i8]) -> i8 {
        let mut sum: i8 = 0;

        for number in array {
            sum += number
        }

        sum
    }

    let sum_result = add_func(&[1, 2, 3]);
    println!("Sum result is {sum_result}"); */

    /*
       EXERCISE 1
    */
    /* let array_need_find_max: &[i32] = &[9, 66, 96, 1, -11, 0];
    fn find_max(array: &[i32]) -> &i32 {
        let mut max: &i32 = &0;

        for number in array {
            if number > max {
                max = number;
            }
        }

        max
    }
    let max_number = find_max(array_need_find_max);
    println!("Max number is {max_number}"); */

    /*
       EXERCISE 2
    */
    /* let array_need_to_find_even: &[i32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    fn filter_even_array(array: &[i32]) -> Vec<&i32> {
        let mut result_array: Vec<&i32> = vec![];

        for number in array {
            if number % 2 == 0 {
                result_array.push(number)
            }
        }

        result_array
    }
    let filltered_array = filter_even_array(array_need_to_find_even);
    println!("Filtered array is {:?}", filltered_array); */

    // CLOSURES
    /* fn closure_power(x: u32) -> impl Fn(i32) -> i32 {
        move |number: i32| number.pow(x)
    }

    let square_func = closure_power(2);

    println!("Square of 5: {}", square_func(5)); */
    /*
       EXERCISE 1
    */
    let array_need_find_max_closure = [9, 66, 96, 1, -11, 0];
    let mut max_number_closure: &i32 = &0;
    array_need_find_max_closure.iter().for_each(|number| {
        if number > max_number_closure {
            max_number_closure = number;
        }
    });
    println!("Max number is {max_number_closure}");

    /*
       EXERCISE 2
    */
    let array_need_to_find_odd: &[i32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut filltered_array_closure: Vec<&i32> = vec![];
    array_need_to_find_odd.iter().for_each(|number| {
        if number % 2 != 0 {
            filltered_array_closure.push(number)
        }
    });
    println!("Filtered array is {:?}", filltered_array_closure);

    println!("----- End of main func! -----");
}
