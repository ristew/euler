use std::num::Int;

pub fn solve () -> usize {
	if ispythag(3, 4, 5) {
		println!("test works");
	}
	for a in 1..1000 {
		for b in a..1000 {
			let c = 1000 - a - b;
			println!("{}, {}, {}", a, b, c);
			if ispythag(a, b, c) {
               	return a * b * c; 
			}
		}
	}
	return 0;
}

fn ispythag(a: usize, b: usize, c: usize) -> bool {
	if a.pow(2) + b.pow(2) == c.pow(2) {
		return true;
	}
	false
}
