use std::io;
extern crate regex;
use regex::Regex;

fn main() {
    let mut tl = String::new();     // TokenList
    println!("式を入力してください");
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

    // vec→可変長配列.f64型として定義.
    let mut stack: Vec<f64> = Vec::new();

    // マッチした箇所全体を取り出す
    let re = Regex::new(r"[0-9]+").unwrap();

    // tlが空白で分割できる限り,分割してできたtokenに対して以下の処理
    for token in tl.split_whitespace(){

        match token{
            // 演算子
            "+" => add(&mut stack),
            "-" => substract(&mut stack),
            "/" => divide(&mut stack),
            "*" => multiply(&mut stack),
            // 半角数字の場合,stackにプッシュ
            _ => match re.captures(token).unwrap().at(0){
                    None => println!(""),   
                    Some(numeric) => {
                            let num: f64 = numeric.parse().unwrap(); 
                            stack.push(num);
                    },
              }
        }
    }
    stack.pop().unwrap()     // 計算結果を取り出す.
}

fn add(stack: &mut Vec<f64>){
    let x:f64 = stack.pop().unwrap();
    let y:f64 = stack.pop().unwrap();
    let z = y + x;
    stack.push(z);
}

fn substract(stack: &mut Vec<f64>){
    let x:f64 = stack.pop().unwrap();
    let y:f64 = stack.pop().unwrap();
    let z = y - x;
    stack.push(z);				
}

fn multiply(stack: &mut Vec<f64>){
    let x:f64 = stack.pop().unwrap();
    let y:f64 = stack.pop().unwrap();
    let z = y * x;
    stack.push(z);	
}

fn divide(stack: &mut Vec<f64>){
    let x:f64 = stack.pop().unwrap();
    let y:f64 = stack.pop().unwrap();
    let z = y / x;
    stack.push(z);
}