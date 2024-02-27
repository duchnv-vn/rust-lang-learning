pub(crate) fn main() {
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
    /* let array_need_find_max_closure = [9, 66, 96, 1, -11, 0];
    let mut max_number_closure: &i32 = &0;
    array_need_find_max_closure.iter().for_each(|number| {
        if number > max_number_closure {
            max_number_closure = number;
        }
    });
    println!("Max number is {max_number_closure}"); */

    /*
       EXERCISE 2
    */
    /* let array_need_to_find_odd: &[i32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut filltered_array_closure: Vec<&i32> = vec![];
    array_need_to_find_odd.iter().for_each(|number| {
        if number % 2 != 0 {
            filltered_array_closure.push(number)
        }
    });
    println!("Filtered array is {:?}", filltered_array_closure); */
}
