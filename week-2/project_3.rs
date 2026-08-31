fn main() {
	let p:f64 = 210_000.0;
	let n:f64 = 3.0;
	let r:f64 = 5.0;

	// depriciation amount 
	let dep = p * (1.0 - (r / 100.0)).powf(n);
	println!("Depriciated amount is {:.2}", dep);
}