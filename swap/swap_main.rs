/*
 Rust
 入力した二つの数値を交換
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
    print!("a = ");
    io::stdout().flush().unwrap();
    let a:i32 = read();
    
    print!("b = ");
    io::stdout().flush().unwrap();
    let b:i32 = read();

    // 同じ名前の変数を後から導入した場合、実際にその名前で使用できる変数は最後に導入した変数になる（シャドウイング）
    let tmp = b;
    let b = a;
    let a = tmp;
    
    /* i32型はCopyトレイトを実装しているため、let a = b; と記述しても所有権の移動は起こらず値がコピーして渡される。
        Copyトレイト実装には条件があり、下記三つすべての条件を満たすとき実装可能である。
        ・条件１：その型（構造体や列挙型）のすべてのフィールドの型がCopyトレイトを実装している。
        ・条件２：その型自身とすべてのフィールドの型がデストラクタ（Dropトレイト）を実装していない。
                　たとえばBox<T>型、Vec<T>型、String型はデストラクタを持つため、フィールドにそれらの型をもつときはCopyトレイトは実装不可能である。
        ・条件３：その型自身がCloneトレイトを実装している。
        
        型ごとに所有権が移動するか否か確かめて代入文を記述する必要がある？
    */

    println!("===== After =====");
    println!("a = {}", a);
    println!("b = {}", b);
}
