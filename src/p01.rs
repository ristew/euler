pub fn solve(max: uint) -> uint {
	let mut tot: uint = 0;
	for i in 1..max {
		if (i%3==0 || i%5==0) {
			tot += i;
		}
	}
	tot
}
