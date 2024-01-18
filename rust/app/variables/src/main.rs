/*
* 定数の定義例
*
* 定数はグローバルスコープに定義することも可能
*
* 定数は、型を明示する必要があり、
* 命名規則は、すべて大文字で、
* 単語間をアンダースコアで区切る
*
* 定数は、定数式のみで初期化可能
* 定数は、mutキーワードを使用することはできない
* 定数は、シャドーイングすることはできない
*
* 数値は、数値リテラルの間にアンダースコアを使用して、
* 100_000のように、視覚的に見やすくすることができる
*/
const MAX_POINTS: u32 = 100_000;

fn main() {

    /*
    * `let x = 5;`として、
    * mut キーワードを使わない場合、
    * 変数はデフォルトで不変なため、
    * コンパイルエラーになります。
    */
    let mut x = 5;
    println!("The value of x is: {}", x);   // xの値は5です
    x = 6;
    println!("The value of x is: {}", x);   // xの値は6です
    x += 1;
    println!("The value of x is: {}", x);   // xの値は7です
    x *= 2;
    println!("The value of x is: {}", x);   // xの値は14です
    x /= 2;
    println!("The value of x is: {}", x);   // xの値は7です
    x -= 1;
    println!("The value of x is: {}", x);   // xの値は6です

    /*
    * `let y = 5;`として、
    * mut キーワードを使わない場合、
    * 変数は不変ですが、
    * 再度letキーワードを指定し、
    * 変数をシャドーイングすることで
    * 同じ変数名の変数を再定義することは可能です。
    * 
    * シャドーイングの際は、`+=` のような
    * 複合代入演算子は使用できません。
    * 
    * また、シャドーイングの際は、
    * 型を変更することも可能です。
    * ですが、mutキーワードを使用する場合、
    * 同じ変数名の変数を再定義することはできません。
    *
    * 他にも、あるスコープ内でシャドーイングした変数は、
    * そのスコープ内でのみ有効で、
    * スコープを抜けると、元の変数が有効になります。
    */
    let y = 5;
    println!("The value of y is: {}", y);   // yの値は5です
    let y = y + 1;
    println!("The value of y is: {}", y);   // yの値は6です

    {
        let y = y * 2;
        println!("The value of x in the inner scope is: {}", y);
    }

    println!("The value of x is: {}", y);   // yの値は6です

    // シャドーイングによる型変換
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);   // spacesの値は3です

    // 定数の出力
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);   // MAX_POINTSの値は100000です
}
