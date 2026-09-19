use std::io;

fn main() {
    let mut experience_input = String::new();
    let mut age_input = String::new();

    // Ask if the employee is experienced
    println!("Is the employee experienced? (enter 'true' or 'false'):");
    io::stdin()
        .read_line(&mut experience_input)
        .expect("Failed to read line");

    let is_experienced: bool = experience_input
        .trim()
        .parse()
        .expect("Please enter true or false");

    // Ask for the employee's age
    println!("Enter the employee's age:");
    io::stdin()
        .read_line(&mut age_input)
        .expect("Failed to read line");

    let age: u32 = age_input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    // Calculate incentive based on criteria
    let incentive: u32;

    if is_experienced {
        if age >= 40 {
            incentive = 1_560_000;
        } else if age >= 30 && age <= 39 {
            incentive = 1_480_000;
        } else if age < 28 {
            incentive = 1_300_000;
        } else {
            // Default fallback for experienced employees aged 28 or 29
            incentive = 1_300_000; 
        }
    } else {
        incentive = 100_000;
    }

    println!("The annual incentive is: N{}", incentive);
}