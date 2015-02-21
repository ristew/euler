pub fn solve() -> u64 {
	let mut tri = 1;
	let mut i = 1;
	while(numdivisors(tri) < 500) {
		i += 1;
		tri += i;
		
	}
	tri
}

fn numdivisors (n: u64) -> u64 {
	let mut tot: u64 = 0;
	for i in 1..n { 
		if n%i==0 {
			tot += 1;
		}
	}
	println!("{}", tot);
	tot
}
