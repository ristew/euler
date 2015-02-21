use std::num::Int;

pub fn solve() -> usize {
	let mut tot = 0;
	let mut sq = 0;
	for i in 1..101 {
		tot += i.pow(2);
		sq += i;
	}
	tot = sq.pow(2) - tot;
	tot
}
