use std::vec::Vec;

pub fn solve() -> u64 {
	let mut p: u64 = 2;
	let mut cnt = 0;
	let mut tot: u64 = 0;
	let mut primes: Vec<u64> = Vec::new();
	while p < 2000000 {
		primes.push(p);
		tot += p;
		cnt+=1;
		if cnt%1000==0 {
			println!("{}", tot);
		}
		p = nextprime(p, &primes);
	}
	return tot;
}

fn nextprime (n: u64, primes: &Vec<u64>) -> u64 {
	if n==2 {
		return 3;
	}
	let mut i = 0;
	for mut i in n+1..100000*n {
		if isprime(i, primes) {
			return i;
		}
		i += 1;
	}
	return 0;
}
		
fn isprime(n: u64, primes: &Vec<u64>) -> bool {
	for &i in primes.iter() {
		if n%i==0 {
			return false;
		}
	}
	true
}
