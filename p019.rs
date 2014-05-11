static MONTH_DAYS: [uint, ..12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn days_in_month(month: uint, year: uint) -> uint {
	let days = MONTH_DAYS[month];
	if month == 1 && year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
		days + 1
	} else {
		days
	}
}

fn main() {
	let mut day = 0;
	let mut sundays = 0;
	for year in range(1900u, 2001u) {
		for month in range(0u, 12u) {
			if day % 7 == 6 && year > 1900 && year <= 2000 {
				sundays += 1;
			}
			day += days_in_month(month, year);
		}
	}
	println!("{}", sundays);
}
