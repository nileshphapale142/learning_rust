fn main() {
    // this is simple loop;
    loop {
        println!("this is inside of loop");
        break;
    }
    
    println!("Now see the while loop:");
    let mut number = 1;
    
    while number != 5 {
        println!("The numer is {}", number);
        number += 1;
    } 
    
    println!("Let's see the while loop on array");
    
    let arr = [19, 241, 3414, 435, 23,321];
    let mut index = 0;
    while index != arr.len() {
        println!("arr[{}] = {}", index, arr[index]);
        index += 1;
    } 
    
    println!("Now see the for loop  on array");
    
    for element in arr.iter() {
        println!("element is {}", element);
    }
    
   println!("Let's see a count-down using for loop");
   
   for number in (0..11).rev() {
       println!("{}", number);
   }
  println!("LIFTOFF"); 
}