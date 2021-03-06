# C言語 → Rust  変換ルール

### 代入,初期化

C言語
```
int i = 0;
```
Rust
```
let i:i32;
```
- 代入はletを使って記述．基本的にmutを用いる
  - 同じ名前の変数を後から導入した場合、実際にその名前で使用できる変数は最後に導入した変数になる
- Rustは初期化不要

<br>

i32型はCopyトレイトを実装しているため、```let a = b;```と記述しても所有権の移動は起こらず値がコピーして渡される。
Copyトレイト実装には条件があり、下記三つすべての条件を満たすとき実装可能である。
  - 条件１：その型（構造体や列挙型）のすべてのフィールドの型がCopyトレイトを実装している。
  - 条件２：その型自身とすべてのフィールドの型がデストラクタ（Dropトレイト）を実装していない。たとえばBox<T>型、Vec<T>型、String型はデストラクタを持つため、フィールドにそれらの型をもつときはCopyトレイトは実装不可能である。
  - 条件３：その型自身がCloneトレイトを実装している。
        
Copyトレイトが実装されているか否か確かめて代入文を記述する必要がある？

<br>

### 配列

C言語
```
float num[100];
```
Rust
```
let mut num:[型; 配列の数] = [0.0; 100];    // f64型num配列100個
```

<br>

### 関数

C言語
```
返り値の型 関数名(引数){
  関数の中身
}
```

Rust
```
fn 関数名(引数) -> 返り値の型{
  関数の本体
}
```
- 関数内で引数の値を変更する場合，```fn 関数名(mut i:i32,mut n:f64)```といった形でmutをつける

<br>

### 分岐

C言語
```
if(条件式){
  trueの時
}else{
  falseの時
}
```

Rust
```
if   条件式{
  trueの時
}else{
  falseの時
}
```

<br>

### 繰り返し
C言語
```
while(条件式){
  ループ本体
}
```

Rust
```
while 条件式{
  ループ本体 
}
```

<br>

C言語
```
for(i=0; i<99; i++){
  ループ本体
}
```

Rust
```
for i in 0..99{
  ループ本体 
}
```

### ポインタ
C言語
```
int *p;
int k = 12345;
int t;

p = &k;
t = *p;
```

Rust
```
let k:i32 = 12345;
	
let p: *const i32 = &k as *const i32;
let t = unsafe{*p};
```

- ポインタを表示するために使用可能なポインタ型のデフォルトのフォーマッタ　{:p}
- 生ポインタを取得するには、参照からas *const T（イミュータブルの場合）またはas *mut T（ミュータブルの場合）でキャスト
