fn main() {
	let k:i32 = 12345;
	
	// 生ポインタを取得するには、参照からas *const T（イミュータブルの場合）またはas *mut T（ミュータブルの場合）でキャスト 
	let p: *const i32 = &k as *const i32;
	let t = unsafe{*p};

	// ポインタを表示するために使用可能なポインタが他のデフォルトのフォーマッタ　{:p}
	println!( "p={:p}, t={}", p, t);
	
}
