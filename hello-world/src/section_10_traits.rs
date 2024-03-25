struct Phone {
    brand: String,
    model: String,
    name: String,
    price: f64,
    android_version: u8,
}

struct Laptop {
    brand: String,
    model: String,
    name: String,
    price: f64,
    ram: u8,
    insurance_name: String,
}

struct PC {
    brand: String,
    model: String,
    name: String,
    price: f64,
    ram: u8,
    cpu: u8,
}

trait Electronics {
    fn print_all_data(&self);
    fn get_brand(&self) -> String;
}

trait Insurance {
    fn get_insurance_name(&self) -> String;
}

trait Android: Electronics {
    fn get_android_version(&self) -> u8;
}

impl Electronics for Phone {
    fn print_all_data(&self) {
        println!(
            "Data: {}, {}, {}, {}",
            self.brand, self.model, self.name, self.price
        )
    }

    fn get_brand(&self) -> String {
        self.brand.clone()
    }
}

impl Electronics for Laptop {
    fn print_all_data(&self) {
        println!(
            "Data: {}, {}, {}, {}, {}",
            self.brand, self.model, self.name, self.price, self.ram
        )
    }

    fn get_brand(&self) -> String {
        self.brand.clone()
    }
}

impl Insurance for Laptop {
    fn get_insurance_name(&self) -> String {
        self.insurance_name.clone()
    }
}

impl Android for Phone {
    fn get_android_version(&self) -> u8 {
        self.android_version.clone()
    }
}

pub(crate) fn main() {
    let phone_1 = Phone {
        brand: String::from("OPPO"),
        model: String::from("Reno"),
        name: String::from("OPPO Reno 8 128GB"),
        price: 9800000.0,
        android_version: 14,
    };

    let latop_1 = Laptop {
        brand: String::from("HP"),
        model: String::from("Envy"),
        name: String::from("HP Envy 13"),
        price: 21000000.0,
        ram: 8,
        insurance_name: String::from("Insurance AAA"),
    };

    let pc_1 = PC {
        brand: String::from("DELL"),
        model: String::from("Intel core i7"),
        name: String::from("DELL Intel core i7"),
        price: 18000000.0,
        ram: 16,
        cpu: 4,
    };

    // phone_1.print_all_data();
    // latop_1.print_all_data();

    fn print_electronics_brand(product: &impl Electronics) {
        println!("Data: {}", product.get_brand())
    }
    // print_electronics_brand(&latop_1)

    fn print_electronics_brand_2<T: Electronics>(product: &T) {
        println!("Data: {}", product.get_brand())
    }
    // print_electronics_brand_2(&phone_1)

    fn print_electronics_brand_and_insurance(product: &(impl Electronics + Insurance)) {
        println!(
            "Data: {}, {}",
            product.get_brand(),
            product.get_insurance_name()
        )
    }
    // print_electronics_brand_and_insurance(&latop_1)

    // SUPER TRAIT
    fn print_android_phone_version(product: &impl Android) {
        println!("Data: {}", product.get_android_version())
    }
    // print_android_phone_version(&phone_1);

    // RETURN TRAIT
    fn define_new_eletronic(eletronic_type: &str) -> Box<dyn Electronics> {
        match eletronic_type {
            "laptop" => Box::new(Laptop {
                brand: String::new(),
                model: String::new(),
                name: String::new(),
                price: 0.0,
                ram: 0,
                insurance_name: String::new(),
            }),
            _ => Box::new(Phone {
                brand: String::new(),
                model: String::new(),
                name: String::new(),
                price: 0.0,
                android_version: 0,
            }),
        }
    }
    let laptop_2 = define_new_eletronic("laptop");

    // TRAIT OBJECT
    // let vec: Vec<Box<dyn Electronics>> = vec![Box::new(latop_1), laptop_2];
    // vec[0].print_all_data()

    // STATIC DISPATCH
    fn get_brand<T: Electronics>(product: &T) -> String {
        product.get_brand()
    }
    get_brand(&phone_1);

    // DYNAMIC DISPATCH
    fn get_brands(products: &[&dyn Electronics]) {
        for product in products {
            product.get_brand();
        }
    }
    get_brands(&[&latop_1, &phone_1])
}
