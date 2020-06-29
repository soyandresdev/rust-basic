fn main() {
    println!("Arrays");
                // 0,1,2,3,4
    let numbers = [1,2,3,4,5]; // size -> 5
    let numbers_more_explicit: [i32; 5] = [1,2,3,4,5]; // size -> 5
    // println!("The value of array is: {}", numbers) => Get error 
    println!("The value of array is: {:?}", numbers);
    println!("The value of array more explicit is: {:?}", numbers_more_explicit);

    let values  = [5.5; 10];
    println!("The value of array is: {:?}", values);

    let mut numbers = [1,2,3,4,5]; // size -> 5

    let one_element_from_array_is = numbers[0];
    let last_element_from_array_is = numbers[numbers.len() - 1];
    numbers[numbers.len() - 1] = 10;
    println!("Get first value element from array number {}", one_element_from_array_is);
    println!("Get last value element from array number {}", last_element_from_array_is);
    println!("The value of array is: {:?}", numbers);

}
