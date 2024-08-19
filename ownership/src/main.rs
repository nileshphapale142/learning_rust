fn main() {
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moved into function
                        // s no longer valid here
    
    let x = 5; // x comes into scope
    makes_copy(x); // x moves into function but i32 is Copy so scope does not end here
} // x goes outof scope then s. 

fn takes_ownership(str : String) { // str comes into scope
    println!("{}", str);
} // str goes outof scope and drop is called. The backing memory is freed

fn makes_copy(num : i32) { //scope of num starts
    println!("{}", num); 
} // scope of num ends