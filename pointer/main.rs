fn main() {
	let k:i32 = 12345;
	
	let p: *const i32 = &k as *const i32;
	let t = unsafe{*p};

	println!( "p={:p}, t={}", p, t);
}
