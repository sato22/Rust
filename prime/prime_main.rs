/*
 Rust
 1から1000までの素数を表示
*/

fn main() {
    let mut i = 2;
    let mut j = 2;
    
    // 判定するためのフラグ
    let mut flag;
    
    // 素数かどうか判定
    while i <= 1000 {
        flag = 0;
        
        while j < i {
            if i % j == 0{
                flag = 1;
                // println!("flag = 1");
                break;
            }
            j += 1;     // Cにおけるfor文のインクリメント
        }
        
        // 判定結果の出力
        if flag == 0 {
            print!("{} ",i);
        }
        
        j = 2;      // jを初期化
        // println!("i = {}, j = {}",i,j);
        
        i += 1;     // Cにおけるfor文のインクリメント
    }
    
    println!("");
}
