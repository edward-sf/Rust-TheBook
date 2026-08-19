fn main() {
    let s = String::from("Hello, world!");

    // Slicing the string to get "Hello" and "world"
    let hello = &s[0..5];
    let world = &s[7..12];

    println!("{}", hello);
    println!("{}", world);

    let s = String::from("Caboose");

    // Slicing the string at a midpoint
    let half = &s[0..4]; // This will give us "Cabo"
    let half = &s[..4]; // This will also give us "Cabo" since the end index is omitted

    let half2 = &s[4..s.len()]; // This will give us "ose"
    let half2 = &s[4..]; // This will also give us "ose" since the start index is omitted
    
    println!("{}", half);
    println!("{}", half2);

    let slice = &s[0..s.len()]; // This will give us the entire string "Caboose"
    let slice2 = &s[..]; // This will also give us the entire string "Caboose" since both start and end indices are omitted
    println!("{}", slice);
    println!("{}", slice2);

    let s = "Hello, world!"; // String literals ARE slices
                             // They are immutable references with the `&str` type
    println!("{}", first_word(s));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}