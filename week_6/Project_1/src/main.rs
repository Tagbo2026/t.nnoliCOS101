use std::io::{self, Write};

fn main() {
    println!("========== RESTAURANT MENU ==========");
    println!("P - Poundo Yam / Edinkaiko Soup : N3,200");
    println!("F - Fried Rice & Chicken        : N3,000");
    println!("A - Amala & Ewedu Soup          : N2,500");
    println!("E - Eba & Egusi Soup            : N2,000");
    println!("W - White Rice & Stew           : N2,500");
    println!("=====================================");

    // Read the food choice
    print!("\nEnter food code (P, F, A, E, W): ");
    io::stdout().flush().unwrap();
    let mut code_input = String::new();
    io::stdin()
        .read_line(&mut code_input)
        .expect("Failed to read input");

    let code = code_input.trim().to_uppercase();

    // Determine unit price based on the selected letter
    let unit_price: f64 = match code.as_str() {
        "P" => 3200.0,
        "F" => 3000.0,
        "A" => 2500.0,
        "E" => 2000.0,
        "W" => 2500.0,
        _ => {
            println!("Invalid food code entered!");
            return;
        }
    };

    // Read quantity
    print!("Enter quantity: ");
    io::stdout().flush().unwrap();
    let mut qty_input = String::new();
    io::stdin()
        .read_line(&mut qty_input)
        .expect("Failed to read input");

    let quantity: f64 = match qty_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number for quantity!");
            return;
        }
    };

    // Calculate total
    let mut total = unit_price * quantity;
    println!("\nSubtotal: N{:.2}", total);

    // Apply 5% discount if total exceeds N10,000
    if total > 10000.0 {
        let discount = total * 0.05;
        total -= discount;
        println!("Discount (5%): -N{:.2}", discount);
    }

    println!("Total Charge: N{:.2}", total);
}