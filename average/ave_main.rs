/*
 Rust
 配列の数値の平均値を計算
*/

use std::io;
use std::io::Write;

fn main() {
    let mut i;
    let mut n;
    
    let mut num: [f64; 100];    // num配列100個
    
    let mut sum = 0.0;
    let mut ave;
    
    print!("数値の数を入力(1〜100) = ");
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut n).unwrap();    // 一行読む．失敗は無視
    let mut n = n.trim();   // 末尾にある改行コードを削る
    
    while i < n {
        print!("{}個目 = ",i+1);
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut num[i]).unwrap();    // 一行読む．失敗は無視
        num[i] = num[i].trim();   // 末尾にある改行コードを削る
        sum += num[i];
    }
    
    ave = sum/n;
    println!("数値の平均値 = {}",ave);
}
