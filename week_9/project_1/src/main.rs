use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("nigerian_breweries.txt").expect("creation failed");
    
    let drinks_data = "Lager:\n33 Export\nGoldberg\nGulder\nHeineken\nStar\n\nStout:\nLegend\nTurbo King\nWilliams\n\nNon-Alcoholic:\nMaltina\nAmstel Malta\nMalta Gold\nFayrouz\n";
    
    file.write_all(drinks_data.as_bytes()).expect("write failed");
    println!("Nigerian Breweries data written to file.");
}