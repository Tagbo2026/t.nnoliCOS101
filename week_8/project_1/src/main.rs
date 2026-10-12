use std::io;

fn main() {
    let current_utc_hour: u32 = 18; 

    // Fixed warning by removing redundant >= 0 check for unsigned integer
    let time_greeting = if current_utc_hour < 12 {
        "Good morning"
    } else if current_utc_hour < 17 {
        "Good afternoon"
    } else {
        "Good evening"
    };

    println!("{} Mr. Chudi!", time_greeting);
    println!("Current calculated UTC hour: {}\n", current_utc_hour);

    let categories = vec!["Office Admin", "Academic", "Lawyer", "Teacher"];
    
    let levels = vec![
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "EL1 8-10",
        "EL2 10-13",
        "SES",
    ];

    let office_admin_roles = vec![
        "Intern",
        "Administrator",
        "Senior Administrator",
        "Office Manager",
        "Director",
        "CEO",
    ];

    let academic_roles = vec![
        "N/A",
        "Research Assistant",
        "PhD Candidate",
        "Post-Doc Researcher",
        "Senior Lecturer",
        "Dean",
    ];

    let lawyer_roles = vec![
        "Paralegal",
        "Junior Associate",
        "Associate",
        "Senior Associate 1-2",
        "Senior Associate 3-4",
        "Partner",
    ];

    let teacher_roles = vec![
        "Placement",
        "Classroom Teacher",
        "Senior Teacher",
        "Leading Teacher",
        "Deputy Principal",
        "Principal",
    ];

    println!("Welcome to this APS Level Checker App");
    println!("To begin input enter your field of choice:");
    println!("Enter O if you are an office admin");
    println!("Enter A if you are an academic");
    println!("Enter L if you are a lawyer");
    println!("Enter T if you are a teacher");

    let mut field_choice = String::new();
    io::stdin()
        .read_line(&mut field_choice)
        .expect("Failed to read input");
    let field = field_choice.trim().to_uppercase();

    let selected_roles: &[&str];
    let custom_greeting: &str;
    let selected_category: &str;

    if field == "O" {
        selected_category = categories[0];
        selected_roles = &office_admin_roles;
        custom_greeting = "Keeping operations seamless and organized!";
    } else if field == "A" {
        selected_category = categories[1];
        selected_roles = &academic_roles;
        custom_greeting = "Driving research and expanding human knowledge!";
    } else if field == "L" {
        selected_category = categories[2];
        selected_roles = &lawyer_roles;
        custom_greeting = "Upholding justice and advocating with integrity!";
    } else if field == "T" {
        selected_category = categories[3];
        selected_roles = &teacher_roles;
        custom_greeting = "Empowering the next generation of leaders and thinkers!";
    } else {
        println!("Invalid field selection.");
        return;
    }

    println!("\nGreat, you are a {}", selected_category);
    println!("{}", custom_greeting);
    println!("\nNow let's determine your level in the public service");

    for i in 0..selected_roles.len() {
        if selected_roles[i] != "N/A" {
            println!("Enter {} if you are a {}", i, selected_roles[i]);
        }
    }

    let mut level_choice = String::new();
    io::stdin()
        .read_line(&mut level_choice)
        .expect("Failed to read input");

    let level_index: usize = match level_choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    if level_index < selected_roles.len() && selected_roles[level_index] != "N/A" {
        println!("\nIn the Public Service you are at level {}", levels[level_index]);
    } else {
        println!("\nInvalid position selected for this field.");
    }
}