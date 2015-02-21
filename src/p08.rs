use std::old_io::File;
use std::vec::Vec;
use std::str::StrExt;

pub fn solve() -> u64 {
	let mut file = match File::open(&Path::new("bignum")) {
		Err(why) => panic!("{}", why.desc),
		Ok(file) => file,
	};
	let mut contents = match file.read_to_string() {
		Err(why)	=>	panic!("{}", why.desc),
		Ok(contents)=>	contents,
	};
	let zeros: [u64; 13] = [0; 13];
	let mut mults: Vec<u64> = Vec::new();
	mults.push_all(&zeros);
	let mut multtot = 1;
	let mut multmax = 0;
	let s = contents.as_slice();
	for i in contents.chars() {
		let d = match i.to_string().parse::<u64>() {
			Err(why)	=>	continue,
			Ok(d) 		=>	d,
		};
		
		multtot = 1;
		for &j in mults.iter() {
			multtot = multtot * j;
		}
		if multtot > multmax {
				multmax = multtot;
				println!("{}", multtot);
		}
		mults.push(d);
		mults.remove(0);

	}
	multmax
}
