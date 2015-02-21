pub fn solve() -> u64 {
	let mut p = 2;
	for i in 0..10001 {
		p = nextprime(p);
		println!("{}", p);
	}
	p
}

fn nextprime (n: u64) -> u64 {
	for i in n+1..100000*n {
		if isprime(i) {
			return i;
		}
	}
	return 0;
}
		
fn isprime(n: u64) -> bool {
	if n==2 {
		return true;
	}
	for i in 2..(n-1) {
		if n%i==0 {
			return false;
		}
	}
	true
}
