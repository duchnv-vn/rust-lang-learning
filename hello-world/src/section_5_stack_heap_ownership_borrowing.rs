pub(crate) fn main() {
    // 5.1 OWNERSHIP
    /* let var_1 = String::from("banana");
    let var_2 = var_1;

    println!("var = {}", var_2); */ // ownership of `banana` value transfer from var_1 to var_2 -> raise exception if want to print var_1

    // 5.2 BORROWING
    // 5.2.1 SHARED REFERENCE -> &
    /* let var_6 = String::from("pinapple");
    let var_5 = &var_6;
    let var_4 = &var_5;
    print_fruit(&var_6);
    println!("var_6 = {}", var_6); */

    // 5.2.2 WRITE REFERENCE -> &mut | borow to write with the acceptance from the owner
    /* let mut var_7 = String::from("apple");
    write_and_print_fruit(&mut var_7);
    println!("var_7 = {}", var_7); */

    /* let mut var_8 = String::from("cherry");
    let var_10 = &mut var_8;
    var_10.push_str(" lady");
    let var_9 = &var_10;

    println!("vars = {} {}", var_9, var_10); // cannot print var_8 in this line because it is being borrowed by other vars
    println!("vars = {}", var_8); */
}

// fn write_and_print_fruit(str: &mut String) {
//     println!("str 1 = {}", str);
//     str.push_str(" red");
//     println!("str 2 = {}", str);
// }

// fn print_fruit(str: &String) {
//     println!("str = {}", str);
// }
