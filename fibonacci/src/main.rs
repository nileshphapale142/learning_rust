use std::io;

fn main() {
    
    let mut number = String::new();
    
    println!("Let's nth find fibonacci number ");
    println!("Enter a number (n): ");
    io::stdin()
        .read_line(&mut number)
        .expect("Failded to read line");
    
    let number: u32 = match number.trim().parse() {
        Ok(num) => num, 
        Err(_) => {
            println!("Enter a valid number!!!");
            return ;
        }
    };
    
    let nth_fib = fib(number);
    
    println!("Nth fibonacci number is {}", nth_fib);
}

fn fib(num : u32) -> u32 {
    if num == 0 || num == 1{
        num
    } else {
        fib(num - 1) + fib(num - 2)
    }
}