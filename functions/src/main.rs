fn main() {
    println!("Let's see the functions in rust now\n");
    
    first_function();
    function_with_parameters(-321, 112);
    let five = function_with_return();
    println!("The returned value is {}", five);
    
    let square_5 = square(5);
    println!("Square function takes i32 argument and returns i32 squared number");
    println!("The square of 5 is {}", square_5);
}

fn first_function() {
    println!("This is first function");
}

fn function_with_parameters(x: i32, y: u8) {
    println!("This functions takes two parameters, x: i32 = {} and y: u8 = {}", x, y);
}

fn function_with_return () -> u8 {
    println!("This function returns five");
    println!("In rust expressions do not have semicolon and returning is an expression");
    5
}

fn square(num: i32) -> i32 {
    num * num
} 