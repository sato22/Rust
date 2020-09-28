/* Rust
   入力した文字列の文字数を表示（出力）
*/

// 標準入力から一行読み取り、指定の型に変換する関数
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()              // 改行コードを削って返す
}

fn main() {
    println!("文字列を入力してください");
    let s1:String = read();

    let len = calculate_length(&s1);

    // 文字数を表示
    println!("入力した文字列{}は{}文字です。", s1, len);
}

// String型はコピーできないため借用
fn calculate_length(s: &String) -> usize {
    s.len()
}
