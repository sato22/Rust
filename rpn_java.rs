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

struct Type;

trait Calculate<T> {
    fn method(_: T);
}

impl Calculate<(&str,&mut Vec<f64>)> for Type {
    fn method (args: (&str,&mut Vec<f64>)){
         // マッチした箇所全体を取り出す
         let re = Regex::new(r"[0-9]+").unwrap();

        let x:f64 = args.1.pop().unwrap();
        let y:f64 = args.1.pop().unwrap();
            match args.0{
                "+" => args.1.push(x+y),
                "-" => args.1.push(x-y),
                "/" => args.1.push(x/y),
                "*" => args.1.push(x*y),
                _ => match re.captures(args.0).unwrap().at(0){
                    None => println!(""),   
                    Some(numeric) => {
                        let num: f64 = numeric.parse().unwrap(); 
                        args.1.push(num);
                    },
                }
            }
    }
}

fn rpn(tl: &str) -> f64{
    // 逆ポーランド記法の数式を計算する.返り値は符号付き実数

    /* 次のトークンが演算子ならばスタックから二つの数値をポップして，
        演算子に応じた計算結果をスタックにプッシュする．
        次のトークンが数値ならば数値をプッシュする．*/

    // vec→可変長配列.f64型として定義.
    let mut stack: Vec<f64> = Vec::new();


    // tlが空白で分割できる限り,分割してできたtokenに対して以下の処理
    for token in tl.split_whitespace(){
            Type::method((token,&mut stack));
    }
    stack.pop().unwrap()     // 計算結果を取り出す.
}