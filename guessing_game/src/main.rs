use std::io;

fn main() {
    println!("Guess the number!");

    println!("Input your guess");

    let mut guess = String::new();  
    // присвоение переменной значения, mut - изменчивая; 
    // String::new - функция, возвращающшая новый экземпляр string
    // ::new - значит, что пустая переменная будет 

    // ссылки (&) и переменные по умолчанию неизменяемы, поэтому для изменчивости надо написать mut 

    io::stdin() // вызов функции stdin из модуля io 
        .read_line(&mut guess) // обработка ввода данных пользователя //mut guess - куда передается ввод пользователя
        .expect("Failed to read line"); // обработка ошибки

    println!("You guessed: {}", guess); // скобки фигурные для интерполяции
}
