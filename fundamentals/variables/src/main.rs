
fn main() {
    let x = 5; // mut - добавляет изменяемость переменной
    println!("The value of x is {x}");
    let x = x + 1; 
    println!("The value of x is {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // константам надо присвоить значение при создании. Заглавными буквами пишут.

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); 
        // затенение переменной позволяет заменять значения в определенной области видимости, при нем необходимо повторять let.
    }

    println!("The value of x is: {x}");

    // при let мы фактически создаем новую переменную с тем же названием, а значит, можем поменять ей тип данных

    let spaces = "    ";
    let spaces = spaces.len();
    println!("Spaces equal is: {}", spaces);
}
