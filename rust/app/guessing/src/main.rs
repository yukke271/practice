// 標準ライブラリの読み込み
use std::io;
use std::cmp::Ordering;
// 乱数を生成するためのライブラリの読み込み
use rand::Rng;

fn main() {

    // 改行付きで出力
    println!("Guess the number!");

    /*
    * thread_rng() は、現在のスレッドに対して乱数生成器を初期化する
    * gen_range() は、引数の範囲内の乱数を生成する
    * つまり、1以上、101未満の乱数を生成する
    * これは、gen_range(1..=100); と書くことも可能
    */
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is: {}", secret_number); 

    loop {
        println!("Please input your guess.");

        /*
        * 変数の宣言 mut は可変であることを示す (immutable がデフォルト) 
        * let guess = "42"; とすると、guessは不変な変数となる
        * String::new() は空の文字列を生成する    
        * 
        * つまり、
        * 可変変数guessを宣言し、
        * その変数は現時点では新しい空のStringのインスタンスにバインドされている状態
        */
        let mut guess = String::new();

        /*
        * ここで、std::io::stdin() と呼び出しても可能だが、
        * use std::io; としているので、io::stdin() と呼び出せる
        *
        * &mut guess は可変な参照を渡すことを示す
        * つまり、この場合は、guessの値を変更することを許可している
        * 
        * expect() は、io::Result型の値を返す
        * io::Result型は、列挙型で、列挙子はOkとErr がある
        * Okは、処理が成功したことを示し、Errは、処理が失敗したことを示す
        * expect()は、Errの場合に、引数の文字列を表示する
        * つまり、この場合は、データの読み取り失敗時に、"Failed to read line" と表示される
        * ここで、expect()を呼び出さないと、コンパイルは通るが、警告が出る
        */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        /*
        * guessの型をu32に変更している
        * この場合、guessの値が数値でない場合は、エラーとなる
        * 
        * この時、内部ではshadowingという処理が行われている
        * shadowingとは、同じ変数名を再度宣言することで、
        * 以前の変数を隠すことができる機能である
        */
        let guess: u32 = match guess.trim().parse(){
            // 数値が入力された場合は、Ok(num) が返される
            Ok(num) => num,
            // 数値以外が入力された場合は、Err(_) が返される
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };

        // もしくは、println!("You guessed: {}", guess); とすることも可能
        // guess は、{} で囲むことで、変数として扱える
        println!("You guessed: {guess}");     

        /*
        * guess は、String型のインスタンスである
        * しかし、secret_number は、u32型の整数である
        * つまり、型が異なるため、比較することができない
        * そのため、guessをu32型に変換する必要がある
        */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),       
            Ordering::Greater => println!("Too big!"),      
            Ordering::Equal => {
                println!("You win!");
                // break は、ループを抜ける
                // ここで、breakを書かないと、正答した場合でもループが続くようになってしまう
                break;
            },        
        }
    }
}