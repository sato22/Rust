/*
 Rust
 入力した二つの数値を交換
 */

use std::io;
use std::io::Write;

fn main() {
    let mut a = String::new(); 
    
    print!("a = ");
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut a).unwrap();    // 一行読む．失敗は無視
    let mut a = a.trim();   // 末尾にある改行コードを削る
    
    
    let mut b = String::new(); 
    
    print!("b = ");
    io::stdout().flush().unwrap();  // flush()を使ってprint!を即時出力する
    std::io::stdin().read_line(&mut b).unwrap();    // 一行読む．失敗は無視
    let mut b = b.trim();   // 末尾にある改行コードを削る

    let tmp;
    
    tmp = b;
    b = a;
    a = tmp;

    println!("===== After =====");
    println!("a = {}", a);
    println!("b = {}", b);
    
}
