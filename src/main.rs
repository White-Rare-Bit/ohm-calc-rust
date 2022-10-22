use std::{io, collections::HashMap};

fn main() {
	let mut options: HashMap<String, f32> = HashMap::new();

	options.insert("i".to_string(), 0f32);
	options.insert("r".to_string(), 0f32);
	options.insert("v".to_string(), 0f32);

    println!("Ohm's Law Calculator");
	println!("Select two known values:");
	println!("=======Options=======");
	println!("Current : i");
	println!("Resistence : r");
	println!("Voltage : v");
	div();

	menu(&mut options);
	del_blank(options);
}

fn menu(options: &mut HashMap<String, f32>) {

	let mut n = 0;

	while n < 2 {

		let mut opt = String::new();
		
		io::stdin()
    		.read_line(&mut opt)
			.expect("Input v,r,i");

		println!("You selected: {}", opt);
		div();
		// To remove the trailing whitespace (\n) from the user input.
		opt.pop();

		if let std::collections::hash_map::Entry::Occupied(mut opt) = options.entry(opt) {
			let mut opt_val = String::new();
			println!("Enter the value of your selection:");
			div();
			io::stdin().read_line(&mut opt_val).expect("test");
			div();
			opt.insert(opt_val.trim().parse::<f32>().unwrap());
			n += 1;
		} else {
			println!("Enter a valid option.");
		}
	}
}

fn del_blank (options: HashMap<String, f32>) {
	let mut vars = options.clone();
	for (key, value) in &options {
		if value == &mut 0.0 {
			vars.remove_entry(key);
		}

	}
	match_vars(vars);
}

fn match_vars (vars: HashMap<String, f32>)
{
	if vars.contains_key("i") && vars.contains_key("r") {
		calc(vars, "volt");
	} else if vars.contains_key("v") && vars.contains_key("r") {
		calc(vars, "res");
	} else if vars.contains_key("i") && vars.contains_key("v") {
		calc(vars, "cur");
	}
}

fn calc(vars: HashMap<String, f32>, target: &str) {

	match target {
		"volt" => {
			let i = vars.get("i").unwrap();
			let r = vars.get("r").unwrap();
			let v = i * r;
			println!("The voltage in Volts (V) is calculated to be: {}", v);
		},
		"res" => {
			let i = vars.get("i").unwrap();
			let v = vars.get("v").unwrap();
			let r = v / i;
			println!("The resistance in Ω (ohms) is {}", r);
		},
		"cur" => {
			let r = vars.get("r").unwrap();
			let v = vars.get("v").unwrap();
			let i = v / r;
			println!("The current in amperes (A) is {}", i);
		},
		_ => println!("Fail."),
	}
}

fn div() {
	println!("---------------");
}