

fn nextfib((n, p): (uint, uint)) -> uint {
	n + p
}

pub fn solve (max: uint) -> uint {
	let mut tot: uint = 0;
	let mut fibber: (uint, uint) = (1, 1);
	let k: uint = 0;
	while k < max {
		let (_, p) = fibber;
		fibber = (p, nextfib(fibber));
		let (_, k) = fibber;
		if k%2==0 {
			tot += k;
			println!("{}", k);
		}
		if k > max {
			break;
		}
	}
	tot
}
