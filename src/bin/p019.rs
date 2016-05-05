static MONTH_DAYS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn days_in_month(month: u32, year: u32) -> u32 {
	let days = MONTH_DAYS[month as usize];
	if month == 1 && year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
		days + 1
	} else {
		days
	}
}

fn main() {
	let mut day = 0;
	let mut sundays = 0;
	for year in 1900..2001 {
		for month in 0..12 {
			if day % 7 == 6 && year > 1900 && year <= 2000 {
				sundays += 1;
			}
			day += days_in_month(month, year);
		}
	}
	println!("{}", sundays);
}
