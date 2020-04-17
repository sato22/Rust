use std::io;

fn main() {
    let mut tl = String::new();     // TokenList
    io::stdin().read_line(&mut tl).expect("Failed to read line");      // 標準入力を読み込む
    let tl = tl.trim();     // "\n"を取り除く

    let ans = rpn(tl);     // result

    println!("Tokenlist = {}, Answer = {}", tl, ans);
}

fn rpn(tl: str) -> isize{
    // 逆ポーランド記法の数式を計算する.返り値は符号付き整数
    // vec→可変長配列."mut"により可変な変数であることを宣言.
    let mut stack = Vec::new();     
    
    // tlを空白で分割する. token.next()で一文字進めることができる.
    let mut token = tl.split_whitespace();

    /* 次のトークンが演算子ならばスタックから二つの数値をポップして，
        演算子に応じた計算結果をスタックにプッシュする．
        次のトークンが数値ならば数値をプッシュする．*/

    // for文 tlが分割できる限り,分割してできたtokenに対して以下の処理
        if /* numeric */{
            stack.push(token);
        }else /* not numeric */{
            match {
                "+" => add();
                "-" => substract();
                "/" => divide();
                "*" => multiply();
                _ => println!("Not an operator");      // 例外処理
            }
        }

    stack.pop()     // 計算結果を取り出す.
}

fn add(){
    let x = stack.pop();
    let y = stack.pop();
    let z = y + x;
    stack.push(z);
}

fn substract(){
    let x = stack.pop();
    let y = stack.pop();
    let z = y - x;
    stack.push(z);				
}

fn multiply(){
    let x = stack.pop();
    let y = stack.pop();
    let z = y * x;
    stack.push(z);	
}

fn divide(){
    let x = stack.pop();
    let y = stack.pop();
    let z = y / x;
    stack.push(z);
}
