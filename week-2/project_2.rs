fn main () {
	let t:f64 = 450_000.0 * 2.0;
	let m:f64 = 1_500_000.0;
	let h:f64 = 750_000.0 * 3.0;
	let d:f64 = 2_850_000.0 * 3.0;
	let a:f64 = 250_000.0;

	// sum
	let sum = t + m + h + d + a;
	println!("Sum is {:.2}", sum);

	// avg
	let avg = (t + m + h + d + a)/10.0;
	println!("Average of the following is {:.2}", avg);
	}