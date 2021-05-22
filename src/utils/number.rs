pub fn format_number_scientific(number: f64) -> String {

	if number >= 1000f64 {

		let log_10 = number.log10() as i32;
		let number = (number / 10f64.powi(log_10 - 3)).floor() / 1000f64;

		let mut string = String::new();
		string.push_str(&number.to_string());
		string.push_str("e");
		string.push_str(&log_10.to_string());
		string

	} else {

		let number = (number * 1000f64).floor() / 1000f64;
		number.to_string()

	}

}