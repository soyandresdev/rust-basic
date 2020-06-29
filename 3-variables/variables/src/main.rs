
// Line Comment
/*
Multi-Comment
1
2
3
4
5
*/

// Variables are inmutable

fn main() {
    let name = "Andres Hernandez";
    let age = 25;
    let number_one= 12;
    let number_two= 12;
    let result= number_one + number_two;
    println!("Hello my name is {}, I'm {}!", name, age);
    println!("The result number is: {}", result);
    // Variables Type
    // let name_variable: <T> = <value>;
    // Mutable Variable
    let mut number_three: i32 = 25;
    number_three = 30;

    println!("Number Mut: {}", number_three);
    // Constanst
    /*
        const or static
        cosnt name_variable: <T> = <value>;
    */
    const PI: f32 = 3.1415;
    
    println!("Value pi: {}", PI);
}
