fn main() {
    println!("Section => operators");
    let number_1 = 10;
    let number_2 = 5;
    let number_3 = 1;

    // addition
    let sum = number_1 + number_2;

    // subtraction
    let difference = number_1 - number_3;

    // multiplication
    let product = number_1 * number_2;

    // division
    let quotient = number_1 / number_2;

    // remainder
    let remainder = number_1 % 3;
    println!("Numeric Operations");
    println!("Sum => {}+{} => {}",number_1 , number_2, sum);
    println!("Subtraction => {}-{} => {}",number_1 , number_3, difference);
    println!("Multiplication => {}*{} => {}",number_1 , number_2, product);
    println!("Division => {}/{} => {}",number_1 , number_2, quotient);
    println!("Remainder => {}%{} => {}",number_1 , 3, remainder);

    // Relational operators
    println!("Section -> Relational operators");
    println!("! -> !exp -> Not {}", !false);
    println!("!= -> var != expr -> PartialEq {}", 5 != 5);
    println!("< -> var < var -> PartialOrd {}", 3 < 5);
    println!("<= -> var <= var -> PartialOrd {}", 3 <= 3);
    println!("== -> var == var -> PartialEq {}", 3 == 3);
    println!(">= -> var >= var -> PartialOrd {}", 3 >= 3);
    println!("> -> var > var -> PartialOrd {}", 3 > 3);
    // Logic operators
    println!("Section -> Logic operators");
    println!("|| -> expr || expr ->  {}", false || true);
    println!("&& -> var && expr -> BitAnd {}", false && true);
}
