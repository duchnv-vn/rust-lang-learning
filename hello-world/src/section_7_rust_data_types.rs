// #[derive(Debug)]
// enum Dỉrection {
//     LEFT,
//     RIGHT,
//     TOP,
//     BOTTOM,
// }

// enum Star {
//     Diamond = 1000,
//     Gold = 100,
//     Silver = 10,
//     Copper = 1,
// }

// enum NumberTypes {
//     U32(u32),
//     I32(i32),
// }

// pub struct Person {
//     pub first_name: String,
//     pub last_name: String,
//     pub age: u8,
// }

// struct Laptop {
//     pub brand: String,
//     pub model: String,
//     pub ram: u8,
//     pub memory: u16,
//     pub core: u8,
// }

pub(crate) fn main() {
    // ENUM
    // let use_direction = Dỉrection::TOP;
    // let _next_direction = match_direction(&use_direction);
    // println!("use up: {:?}", next_direction);

    // use Star::*;
    // let start_vex = vec![Diamond, Gold, Silver, Copper];
    // for star in start_vex {
    //     match star as u32 {
    //         size if size >= 1000 => println!("This is the biggest start"),
    //         size if size >= 100 => println!("This is a good-sized start"),
    //         size if size >= 10 => println!("This star is pretty good"),
    //         _ => println!("Not the good-sized start"),
    //     }
    // }

    // let _my_numbers = vec![set_number_type(100), set_number_type(-10)];

    // ---------------------------------------------------------
    // STRUCT
    // let mut person_1 = Person {
    //     first_name: String::from("Duc"),
    //     last_name: String::from("Huynh"),
    //     age: 18,
    // };

    // println!("person first name 1: {}", person_1.first_name);

    // person_1.age = 25;

    // println!("person first name 2: {}", person_1.first_name);

    // ---------------------------------------------------------
    // IMPL
    // impl Laptop {
    //     fn get_brand(&self) -> String {
    //         self.brand.to_string()
    //     }

    //     fn get_ram(&self) -> String {
    //         self.ram.to_string()
    //     }

    //     fn set_ram(&mut self, value: u8) {
    //         self.ram = value;
    //     }
    // }

    // let mut macbook_m1 = Laptop {
    //     brand: String::from("Apple"),
    //     model: String::from("Macbook M1 2023"),
    //     ram: 16,
    //     memory: 512,
    //     core: 10,
    // };

    // println!("Brand name of macbook M1 1: {}", macbook_m1.get_ram());
    // macbook_m1.set_ram(32);
    // println!("Brand name of macbook M1 2: {}", macbook_m1.get_ram())
}

// fn set_number_type(input: i32) -> NumberTypes {
//     let number = match input.is_positive() {
//         true => NumberTypes::U32(input as u32),
//         false => NumberTypes::I32(input),
//     };
//     number
// }

// fn match_direction(direction: &Dỉrection) -> &str {
//     let next_direction = match direction {
//         Dỉrection::LEFT => "Robot go left",
//         Dỉrection::RIGHT => "Robot go right",
//         Dỉrection::TOP => "Robot go top",
//         Dỉrection::BOTTOM => "Robot go dowm",
//     };
//     next_direction
// }
