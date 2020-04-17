// 指定された文字列を標準入力で入力し,合致しているか確かめる
use std::io;
fn main() {
    let mut exp = String::new();
    let ans = "1 2 + 3 *";
    println!("「1 2 + 3 *」と入力してください");

    io::stdin().read_line(&mut exp).expect("Failed to read line"); 
    // ユーザーからの標準入力を読み込む
    let exp = exp.trim();     
    // Stringのtrim()メソッドにより"\n"を取り除く

    if exp == ans{
        println!("OK!");   
    }else{
        println!("input = {}\nanswer = {}\nNG", exp, ans);
    }
}