fn main() {
    println!("KEY POINTS:
--> one mutable reference to a particular scope
--> immutable and mutable references can not occur in the same scope
--> can't create dangling pointers
");
    
    let str = String::from("This is a string");
    let str_len = calculate_length(&str);
    println!("Length of the string \"{}\" is {}", str, str_len);
    
    let mut str2 = String::from("hello");
    println!("\nString is \"{}\"", str2);
    change_string(&mut str2);
    println!("Modifid String is \"{}\"", str2);
    
    
}


fn calculate_length(str : &String) -> usize { //takes a unmutable reference
    str.len()
}

fn change_string(str : &mut String) { //takes a mutable reference
    str.push_str(", whoever is reading");
}

fn dangle() -> &String { // can't create dangling pointers, it will work if ownership is moved instead of reference
    let str = String::from("hello");
    
    &str;
}