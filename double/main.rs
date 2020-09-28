/*
 Rust
 数値の2倍を計算する関数
*/

// 標準入力から一行読み取り、指定の型に変換する関数
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()              // 改行コードを削って返す
}


fn main() {
    println!("数値を入力してください");
    let s:i32 = read();                             //数値の入力
    let n:i32 = var(s);                                 //関数var()を呼び出し、引数sを2倍する
    println!("入力した数値の2倍は{}です", n);
    println!("元の値は{}です", s);
}

fn var(i:i32) -> i32{
    let ans = i * 2;                                //変数を2倍する計算式
    return ans;                                     //ansを返す
}
