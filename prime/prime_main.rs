/*
 Rust
 1から1000までの素数を表示
*/

fn main() {
    let i = 2;
    let mut j = 2;
    
    // 判定するためのフラグ
    let mut flag;
    
    /*
    // 素数かどうか判定 while文
    while i <= 1000 {
        let i = i+1;     // Cにおけるfor文のインクリメント
        flag = 0;
        
        while j < i {
            if i % j == 0{
                flag = 1;
                break;
            }
            j += 1;     // Cにおけるfor文のインクリメント
        }
        
        // 判定結果の出力
        if flag == 0 {
            print!("{} ",i);
        }
        
        j = 2;      // jを初期化
    }
    */
    
    /* 素数かどうか判定　for文
     for n in a..b　で記述。（aからbの一つ前まで）
     全てwhileで記述？
    */
    for i in 2..1001{
        flag = 0;
        
        while j < i{
            if i % j == 0{
                flag = 1;
                break;
            }
            j += 1;     // Cにおけるfor文のインクリメント
        }
        
        // 判定結果の出力
        if flag == 0 {
            print!("{} ",i);
        }
        
        j = 2;      // jを初期化
    }
    
    println!("");
}
