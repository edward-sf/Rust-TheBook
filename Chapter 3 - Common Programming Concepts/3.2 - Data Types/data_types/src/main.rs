fn main() {
    // integer
    let int = 42; // i32 by default
    println!("The value of int is: {}", int);

    // unsigned integer
    let uint: u32 = 42;
    println!("The value of uint is: {}", uint);

    // 64-bit integer
    let int64: i64 = 42;
    println!("The value of int64 is: {}", int64);

    // floating-point number
    let float = 3.14; // f64 by default
    println!("The value of float is: {}", float);

    // 32-bit floating-point number
    let float32: f32 = 3.14;
    println!("The value of float32 is: {}", float32);

    // boolean
    let t = true;
    println!("The value of t is: {}", t);

    // explicitly typed boolean
    let f: bool = false;
    println!("The value of f is: {}", f);

    // character
    let c = 'z';
    println!("The value of c is: {}", c);

    // explicitly typed character
    let z: char = 'ℤ';
    println!("The value of z is: {}", z);

    // Unicode character
    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    // tuple
    let tuple: (i32, f64, u8) = (500, 6.4, 1);

    // destructuring a tuple
    let (x, y, z) = tuple;
    println!("The value of y is: {}", y);

    // indexing a tuple
    let five_hundred = tuple.0;
    println!("The value of five_hundred is: {}", five_hundred);

    // array type
    let months = ["January", "February", "March", "April", "May", "June", "July",
                "August", "September", "October", "November", "December"];
    println!("The value of months is: {:?}", months);

    // explicitly typed array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a is: {:?}", a);

    // array with repeated values
    let b = [3; 5]; // same as [3, 3, 3, 3, 3]
    println!("The value of b is: {:?}", b);

    // indexing an array
    let first = a[0];
    println!("The array's first element is: {}", first);

    // reverse indexing an array
    let last = a[a.len() - 1]; // Rust does not support direct negative indexing, so we use a.len() - 1 to get the last index
    println!("The array's last element is: {}", last);
}
