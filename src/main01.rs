use std::io;//標準ライブラリ（std）から出/入力ライブラリ(io)の導入
use std::cmp::Ordering;//Less,Greater,Equalの中の２値を比較して答えを返してくれる
use rand::Rng;//乱数生成器を使えるようにしてくrているらしい

fn main() {//プログラムが始まることを宣言　（）は謎　＜引数（関数に入れる値）がないってこと！
    println!("Guess the number!");//Guess the numberの表示
let ramdom = rand::thread_rng();
ra
    let secret_number = rand::thread_rng().gen_range(1,101);//1〜100までの適当な数を設定してくれる
   //変数secret_number（不変）の作成、
    

    loop {//この{}の中身を繰り返すよってこと
        
    

    println!("Please input your guess.");//Please input ~の表示

    let mut guess = String::new();//guessという可変な変数の導入　String::new();は難しい　 Stringは形の様式だ,new = Stringに対しての新しく文字列入れれるよっていう関連関数.つまり数字当てげーむの、予想する数字を入れられるようにする部分か



    io::stdin()//わからない　ioを使ってターミナルになんか出すよってこと？

    .read_line(&mut guess)//書かれた推測文字を文字列として置くもの？
    .expect("Failed to read line");//わからん


    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,//！この辺難しい！
    };

    

    println!("You guessed: {}",guess);//You guess、入力されたguessの表示、

    match guess.cmp(&secret_number) {//guessとsecret_numberの比較、Less.Greater.Equalのどれかを出す
        Ordering::Less => println!("Too small!"),//Too smallの表示
        Ordering::Greater => println!("Too big!"),//Too bigの表示
        Ordering::Equal => {
            println!("You win!");//You winの表示
            break;//matchの終わり
        }

    }
  }
}
//スコープに導入って何、スコープってどこ



