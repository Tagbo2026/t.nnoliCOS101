use std::io::{self, Write};

fn read_f64(prompt: &str) -> f64 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
        .trim()
        .parse::<f64>()
        .expect("Please enter a valid number")
}

fn main() {
    println!("--- Quadratic Equation Solver ---");

    let a = read_f64("Enter value for a: ");
    let b = read_f64("Enter value for b: ");
    let c = read_f64("Enter value for c: ");

    if a == 0.0 {
        println!("This is not a quadratic equation (a cannot be 0).");
        return;
    }

    // Discriminant formula: d = b^2 - 4ac
    let d = b * b - 4.0 * a * c;

    println!("\nDiscriminant (d) = {}", d);

    if d > 0.0 {
        // Two distinct real roots
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        println!("The equation has two distinct real roots:");
        println!("Root 1 = {:.4}", root1);
        println!("Root 2 = {:.4}", root2);
    } else if d == 0.0 {
        // One real root (repeated)
        let root = -b / (2.0 * a);
        println!("The equation has exactly one real root:");
        println!("Root = {:.4}", root);
    } else {
        // No real roots
        println!("The equation has no real roots.");
    }
}