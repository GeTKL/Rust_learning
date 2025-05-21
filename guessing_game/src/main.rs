use rand::Rng;
use std::{cmp::Ordering, io};
// cargo doc --open - открыть документацию, клевая штука

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::rng().random_range(1..=100); // генерация рандомного числа от 1 до 100

    loop {
        //бесконечный цикл
        println!("Input your guess");

        let mut guess = String::new();
        // присвоение переменной значения, mut - изменчивая;
        // String::new - функция, возвращающшая новый экземпляр string
        // ::new - значит, что пустая переменная будет

        // ссылки (&) и переменные по умолчанию неизменяемы, поэтому для изменчивости надо написать mut

        io::stdin() // вызов функции stdin из модуля io
            .read_line(&mut guess) // обработка ввода данных пользователя //mut guess - куда передается ввод пользователя
            .expect("Failed to read line"); // обработка ошибки

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // u32 - 32-разрядное число без знака. i32 = 32-разрядное число
        // trim - удаляет все пробелы
        // parse - преобразовывает строку в другой тип, тип указан после двоеточия
        // возвращает также метод Result, как read_line. Обработка Result идет с помощью except
        // Можем изменить обработку ошибки, как показано выше
        println!("You guessed: {}", guess); // скобки фигурные для интерполяции

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    //Вариант перечисления, cmp - сравнивает, тут 2 значения, guess и secret_number и возвращает 1 из 3 вариантов
    //Match для решения, что делаем дальше
}
