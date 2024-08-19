fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    
    if number % 2 == 0 {
        println!("The number is divisible by 2");
    } else if number % 3 == 0 {
        println!("The number is divisible by 3");
    } else {
        println!("The numebr is neither divisible by 2 nor 3");
    } 
}