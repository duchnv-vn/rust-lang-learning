struct Phone {
    brand: String,
    model: String,
    name: String,
    price: f64,
}

struct Laptop {
    brand: String,
    model: String,
    name: String,
    price: f64,
    ram: u8,
}

trait Electronics {
    fn print_all_data(&self);
}

impl Electronics for Phone {
    fn print_all_data(&self) {
        println!(
            "Data: {}, {}, {}, {}",
            self.brand, self.model, self.name, self.price
        )
    }
}

impl Electronics for Laptop {
    fn print_all_data(&self) {
        println!(
            "Data: {}, {}, {}, {}, {}",
            self.brand, self.model, self.name, self.price, self.ram
        )
    }
}

pub(crate) fn main() {
    let phone_1 = Phone {
        brand: String::from("OPPO"),
        model: String::from("Reno"),
        name: String::from("OPPO Reno 8 128GB"),
        price: 9800000.0,
    };

    let latop_1 = Laptop {
        brand: String::from("HP"),
        model: String::from("Envy"),
        name: String::from("HP Envy 13"),
        price: 21000000.0,
        ram: 8,
    };

    phone_1.print_all_data();
    latop_1.print_all_data();
}
