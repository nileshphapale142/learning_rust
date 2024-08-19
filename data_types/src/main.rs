fn main() {
    println!("
        Here are the followign types in rust:\n
        i8\tu8
        i16\tu16
        i32\tu32
        i64\tu64
        isize\tusize (depends on computer architecture)\n
");
    
    println!("
        Here are the integer literals in rust\n
        98_222\t\t: {}
        0xff\t\t: {}
        0o77\t\t: {}
        0b1111_0000\t: {}
        b'A'\t\t: {}
        ", 98_222, 0xff, 0o77, 0b1111_0000, b'A');

    println!("
        The let's see tuples now
        ");
    
    let tuple: (i32, f64, u8) = (500, 4.3, 3);
    
    let (x, y, z) = tuple;
    
    println!("
        The tuple value are (destructing pattern) ( {}, {}, {} )", x, y, z);
    println!("
        The tuple values are (period) ( {}, {}, {} )", tuple.0, tuple.1, tuple.2);

}
