use std::io::Write;

fn main() {
    let students = vec![
        "Oluchi Mordi, ACC10211111, Accounting, 300",
        "Adams Aliyu, ECO10110101, Economics, 100",
        "Shania Bolade, CSC10328828, Computer, 200",
        "Adekunle Gold, EEE11020202, Electrical, 200",
        "Blanca Edemoh, MEE10202001, Mechanical, 100",
    ];

    let mut file = std::fs::File::create("student_records.txt").expect("failed to create file");

    for s in &students {
        println!("{}", s);
        let line = format!("{}\n", s);
        file.write_all(line.as_bytes()).expect("failed to write");
    }

    println!("\nStudent records saved successfully.");
}