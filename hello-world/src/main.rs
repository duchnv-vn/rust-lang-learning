fn main() {
    println!("Hello Rust bootcamp 2024!");

    // VARIABLES
    let _var_a = "Variable A value";

    let mut _var_b = 1;
    _var_b += 1;

    const _PI: f32 = 3.14;
    let _arr = [1, 2, 3, 4, 5];

    // DATA CASTING
    let x: u16 = 300;
    let _y = x as u8;

    let char_1 = 'A';
    let _num_from_char_1 = char_1 as i8;

    let bool_1 = false;
    let _num_from_bool_1 = bool_1 as i8;

    let int_1: u8 = 54;
    let _char_from_int_1 = int_1 as char;
}
