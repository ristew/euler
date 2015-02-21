use std::num::Int;

pub fn solve() -> usize {
	let mut maxp = 0;
	for mut i in 1..300 {
		i = 1000 - i;
		for mut j in 1..300 {
			j = 1000 - j;
			if ispalindrome(i * j) && i * j > maxp {
				maxp = i * j;
			}
		}
	}
	return maxp;
}

fn ispalindrome(p: usize) -> bool {
	let mut n = p;
	let mut r = 0;
	while n > 0 {
		let d = n%10;
		r = 10*r + d;
		n = n/10;
	}
	println!("{}, {}", p, r);
	if p==r {
		println!("palindrome: {}", p);
		return true;
	}
	return false;
}
