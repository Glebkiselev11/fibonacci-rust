use std::io;

fn main() {
	println!("Fibonacci generator 0.1.0");
	println!("How many numbers you want?");
	
	let quantity_numbers: u32 = input();

	fibonacci(quantity_numbers);
}

fn input() -> u32 {
	return loop {
		let mut quantity_numbers = String::new();
		
		io::stdin().read_line(&mut quantity_numbers)
			.expect("Faild to read line");

		let quantity_numbers: u32 = quantity_numbers.trim().parse()
			.expect("Please type a number!");

		if quantity_numbers > 2 {
			break quantity_numbers
		} 
			
		println!("Please try again (need number more than 2)");
		
	};
}

fn fibonacci(number: u32) {
	println!("number : {}", number);
	let mut x = (1, 1);

	for i in 0..number {
		println!("{}: {}", i, x.0);
		x = (x.1, x.0 + x.1)
	}
}

