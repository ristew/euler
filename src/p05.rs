pub fn solve() -> usize {
	let mut i: u64 = 0;
	for i in 1..100000000000 {
		if digdiv(i) {
			return i;
		}
		if i%1000==0 {
			println!("{}", i);
		}
	}
	return 0;
}

fn digdiv(n: usize) -> bool {
	for i in 10..20 {
		if n%i!=0 {
			return false;
		}
	}
	return true;
}
