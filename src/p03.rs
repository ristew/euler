pub fn solve(p: u64) -> u64 {
	return firstprimefactor(p);
}

fn firstprimefactor(n: u64) -> u64 {
	for i in 2..n {
		if n%i==0 {
			return firstprimefactor(n/i);
		}
	}
	n
}

fn isprime(n: u64) -> bool {
	for i in 2..(n-1) {
		if n%i==0 {
			return false;
		}
	}
	true
}
