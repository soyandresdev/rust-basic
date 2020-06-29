fn main() {
    println!("Hello, world!");
    // shadowing
    /*
    
    */
    
    let value: i32 = 10;

    println!("El valor of my variable is: {}", value);
    
    let value = 20;// shadowing-  Destroy the variable and create a new one
    
    println!("El valor of my variable is: {}", value);
}
