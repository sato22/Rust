use std::io;
extern crate regex;
use regex::Regex;

// vec→可変長配列.グローバル変数.
static stack: Vec<f64> = Vec::new();

fn main() {
    let mut tl = String::new();     // TokenList
    io::stdin().read_line(&mut tl).expect("Failed to read line");      // 標準入力を読み込む
    let tl = tl.trim();     // "\n"を取り除く

    let ans:f64 = rpn(tl);     // result

    println!("Tokenlist = {}, Answer = {}", tl, ans);
}

fn rpn(tl: &str) -> f64{
    // 逆ポーランド記法の数式を計算する.返り値は符号付き実数

    /* 次のトークンが演算子ならばスタックから二つの数値をポップして，
        演算子に応じた計算結果をスタックにプッシュする．
        次のトークンが数値ならば数値をプッシュする．*/

    // マッチした箇所全体を取り出す
    let re = Regex::new(r"\d+").unwrap();

    // tlが空白で分割できる限り,分割してできたtokenに対して以下の処理
    for token in tl.split_whitespace(){
        // 半角数字部分のみ取り出す
        let caps = re.captures(token).unwrap();
        // capsに何らかの値が含まれていれば
        match caps.at(0).unwrap(){
            None => println!(""),   
            _ => {
                    let num: f64 = 
            },
        }

        match token{
            // 演算子
            "+" => add(),
            "-" => substract(),
            "/" => divide(),
            "*" => multiply(),
            // 例外処理
            _ => println!("Not an operator"),
        }
    }
    stack.pop().unwrap()     // 計算結果を取り出す.
}

fn add(){
    let x:f64 = stack.pop().unwrap();
    let y:f64 = stack.pop().unwrap();
    let z = y + x;
    stack.push(z);
}

fn substract(){
    let x:f64 = stack.pop().unwrap();
    let y:f64 = stack.pop().unwrap();
    let z = y - x;
    stack.push(z);				
}

fn multiply(){
    let x:f64 = stack.pop().unwrap();
    let y:f64 = stack.pop().unwrap();
    let z = y * x;
    stack.push(z);	
}

fn divide(){
    let x:f64 = stack.pop().unwrap();
    let y:f64 = stack.pop().unwrap();
    let z = y / x;
    stack.push(z);
}