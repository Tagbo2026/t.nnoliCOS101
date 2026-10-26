
struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let laptops = vec![
        Laptop {
            brand: String::from("HP"),
            price: 650_000,
        },
        Laptop {
            brand: String::from("IBM"),
            price: 755_000,
        },
        Laptop {
            brand: String::from("Toshiba"),
            price: 550_000,
        },
        Laptop {
            brand: String::from("Dell"),
            price: 850_000,
        },
    ];

    let mut total = 0;

    for laptop in &laptops {
        let cost = laptop.cost(3);

        println!("Brand: {}", laptop.brand);
        println!("Cost of 3 laptops: N{}", cost);

        total += cost;
    }

    println!("Total cost: N{}", total);
}
