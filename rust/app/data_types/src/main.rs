fn main() {
    // scalar types(スカラー型)
    // プリミティブ型の一つで、単一の値を表す型。

    /*
    * integer types
    *
    * let x: i8 = 2;    // i8
    * let x: u8 = 2;    // u8
    * let x: i16 = 2;   // i16
    * let x: u16 = 2;   // u16
    * let x = 2;        // i32
    * let x: u32 = 2;   // u32
    * let x: i64 = 2;   // i64
    * let x: u64 = 2;   // u64
    * let x: i128 = 2;  // i128
    * let x: u128 = 2;  // u128
    * let x: isize = 2; // isize
    * let x: usize = 2; // usize
    *
    *
    * integer literals
    *
    * Decimal       : 98_222
    * Hex           : 0xff
    * Octal         : 0o77
    * Binary        : 0b1111_0000
    * Byte (u8 only): b'A'
    */
    let x = 2;  // i32
    println!("x = {}", x);

    /*
    * floating-point types
    *
    * let y: f32 = 2.0; // f32
    * let y = 2.0;      // f64
    */
    let y = 2.0; // f64
    println!("y = {}", y);

    /*
    * numeric operations
    *
    * addition(加算)         : +
    * subtraction(減算)      : -
    * multiplication(乗算)   : *
    * division(除算)         : /
    * remainder(剰余算)      : %
    */
    let sum = 5 + 10; // addition
    println!("sum = {}", sum);
    let difference = 95.5 - 4.3; // subtraction
    println!("difference = {}", difference);
    let product = 4 * 30; // multiplication
    println!("product = {}", product);
    let quotient = 56.7 / 32.2; // division
    println!("quotient = {}", quotient);
    let remainder = 43 % 5; // remainder
    println!("remainder = {}", remainder);

    /*
    * boolean type
    *
    * let t = true;
    * let f: bool = false;
    */
    let t = true;
    println!("t = {}", t);

    /*
    * character type
    *
    * let c = 'z';
    * let z = 'ℤ';
    * let heart_eyed_cat = '😻';
    *
    * character literals
    * 文字列型はダブルクォートで囲むが、文字型はシングルクォートで囲む。
    * Rustの文字型は4バイトであり、ASCII文字だけでなく、
    * アクセント付き文字、中国語、日本語、韓国語、絵文字、
    * ゼロ幅スペースなど、Unicodeスカラー値を表すことができる。
    * Unicodeスカラー値は、 U+0000からU+D7FFまでとU+E000からU+10FFFFまでの範囲の値。
    * 更に詳しい内容は、THE BOOKの「8.2.1. 文字型」にて解説される。
    */
    let c = 'z';
    println!("c = {}", c);

    // compound types(複合型)
    // プリミティブ型の一つで、複数の値を1つにまとめる型。

    /*
    * tuple type
    *
    * let tup: (i32, f64, u8) = (500, 6.4, 1);
    * let tup = (500, 6.4, 1);
    *   
    * let (x, y, z) = tup;
    * let x = tup.0;
    * let y = tup.1;
    * let z = tup.2;
    *
    * タプル型は、複数の異なる型の値を1つにまとめることができる。
    * タプル型は、固定長であり、一度定義したら、その長さは変更できない。
    * タプル型は、タプルの各要素に対して、
    * ドット記法を用いて、アクセスすることができる。
    */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup = {:?}", tup);
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);
    
    /*
    * array type
    *
    * let a: [i32; 5] = [1, 2, 3, 4, 5];
    * let a = [1, 2, 3, 4, 5];
    *
    * let first = a[0];
    * let second = a[1];
    *
    * 配列型は、同じ型の値を複数保持することができる。
    * 配列型は、固定長であり、一度定義したら、その長さは変更できない。
    * 配列型は、配列の各要素に対して、
    * インデックスを用いて、アクセスすることができる。
    *
    * 配列型は、ヒープではなく、スタックにデータを保持する。
    * 配列型は、ヒープにデータを保持するベクタ型とは異なる。
    *
    * 配列型は、ヒープにデータを保持するベクタ型よりも、
    * 以下のような場合に有用
    * ・固定長のデータを保持する場合
    * ・データをスタックに保持する場合
    * ・メモリ使用量を抑えたい場合
    * ・速度を重視したい場合
    *
    * 配列型は、ヒープにデータを保持するベクタ型よりも、
    * 以下のような場合に不向き
    * ・可変長のデータを保持する場合
    * ・データをヒープに保持する場合
    * ・メモリ使用量を重視したい場合
    * ・データの追加や削除を行う場合
    *
    * 配列型は、実行時に配列の長さをチェックする。
    * 配列の長さを超えたインデックスを直接指定すると、
    * コンパイルエラーになる。
    * ただし、変数を用いてインデックスを指定する場合は、
    * 実行時に配列の長さをチェックできないため、
    * コンパイルエラーにならない。
    * そのため、配列の長さを超えたインデックスを指定すると、
    * ランタイムエラーになる。
    * (パニックが発生する)
    */
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a = {:?}", a);
    let first = a[0];
    println!("first = {}", first);

    // 下記のコードは、コンパイルできるがパニックする可能性が高い。
    // Rustのエラー処理に関しては、THE BOOKの「9. エラー処理」にて解説される。
    println!("Please enter an array index.");
    let mut index = String::new();
    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
