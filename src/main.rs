use std::io;
fn main() {
    
    println!("Guess the number!");
    
   
   
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");


        let number: f64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => 0.0,
        };
match  number {
    _ if number ==0.0 =>println!("失敗"),
    _ => println!("You guessed: {}",number * 0.1)
        
    }
}

        
        

    





