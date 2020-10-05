/*
 Rust
 配列の数値の平均値を計算
*/

use std::io;
use std::io::Write;

// 標準入力から一行読み取り、指定の型に変換する関数
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()              // 改行コードを削って返す
}

fn main() {
    
    print!("数値の数を入力(1〜100) = ");
    io::stdout().flush().unwrap();
    let n:f64 = read();
    // n → 36行目で被演算子の型をそろえなければならないのでf64型に
    
    let mut num:[f64; 100] = [0.0; 100];    // f64型num配列100個

    let mut sum:f64 = 0.0;
    let mut i:f64 = 0.0;    
    // i → 29行目で型をそろえなければならないのでf64型に
    

    while i < n {
        print!("{}個目 = ",i+1.0);
        io::stdout().flush().unwrap();
        num[i as usize] = read();   // 配列にアクセスする場合、usizeに変換
        sum += num[i as usize];
        i += 1.0;
    }
    
    let ave = sum/n;        // 被演算子の型は揃える必要がある
    println!("数値の平均値 = {}",ave);

    
    /*
        被演算子やwhileの条件式の型は揃える必要がある → MISRA-Cではルールの一つ
    */
    // average(i,n,num,sum);
}

/*
fn average(mut i: f64,n:f64,mut num:[f64; 100],mut sum:f64){
    while i < n {
        print!("{}個目 = ",i+1.0);
        io::stdout().flush().unwrap();
        num[i as usize] = read();   // 配列にアクセスする場合、usizeに変換
        sum += num[i as usize];
        i += 1.0;
    }
    
    let ave = sum/n;        // 被演算子の型は揃える必要がある
    println!("数値の平均値 = {}",ave);
}
*/
