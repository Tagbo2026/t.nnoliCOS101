use std::io::Write;

fn main() {
    let ministers = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let mut file = std::fs::File::create("efcc_merged.txt").expect("failed to create file");

    for i in 0..ministers.len() {
        let line = format!("{} - {} - {}\n", ministers[i], ministries[i], zones[i]);
        print!("{}", line);
        file.write_all(line.as_bytes()).expect("write failed");
    }

    println!("\nEFCC merge complete.");
}