use std::io;

// Helper function to read float inputs from standard input
fn read_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse::<f64>().expect("Please enter a valid number")
}

// 1. Trapezium Area = (height / 2.0) * (base1 + base2)
fn area_trapezium() -> f64 {
    println!("\n--- Area of Trapezium ---");
    let height = read_input("Enter height:");
    let base1 = read_input("Enter base 1:");
    let base2 = read_input("Enter base 2:");
    (height / 2.0) * (base1 + base2)
}

// 2. Rhombus Area = 0.5 * diagonal1 * diagonal2
fn area_rhombus() -> f64 {
    println!("\n--- Area of Rhombus ---");
    let d1 = read_input("Enter diagonal 1:");
    let d2 = read_input("Enter diagonal 2:");
    0.5 * d1 * d2
}

// 3. Parallelogram Area = base * altitude
fn area_parallelogram() -> f64 {
    println!("\n--- Area of Parallelogram ---");
    let base = read_input("Enter base:");
    let altitude = read_input("Enter altitude:");
    base * altitude
}

// 4. Cube Surface Area = 6 * side * side
fn area_cube() -> f64 {
    println!("\n--- Surface Area of Cube ---");
    let side = read_input("Enter side length:");
    6.0 * side * side
}

// 5. Cylinder Volume = pi * radius * radius * height
fn volume_cylinder() -> f64 {
    println!("\n--- Volume of Cylinder ---");
    let radius = read_input("Enter radius:");
    let height = read_input("Enter height:");
    let pi = std::f64::consts::PI;
    pi * radius * radius * height
}

fn main() {
    println!("=== THE SHAPE CALCULATOR ===");
    println!("Select a formula to compute:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Surface Area of Cube");
    println!("5. Volume of Cylinder");

    let choice = read_input("\nEnter choice (1-5):") as i32;

    match choice {
        1 => {
            let result = area_trapezium();
            println!("Result: Area of Trapezium = {:.2}", result);
        }
        2 => {
            let result = area_rhombus();
            println!("Result: Area of Rhombus = {:.2}", result);
        }
        3 => {
            let result = area_parallelogram();
            println!("Result: Area of Parallelogram = {:.2}", result);
        }
        4 => {
            let result = area_cube();
            println!("Result: Surface Area of Cube = {:.2}", result);
        }
        5 => {
            let result = volume_cylinder();
            println!("Result: Volume of Cylinder = {:.2}", result);
        }
        _ => println!("Invalid selection. Please run the program again and pick 1 to 5."),
    }
}