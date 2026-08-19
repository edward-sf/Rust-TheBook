use std::io;

fn main() {
    let mut first = 0;
    let mut second = 1;

    println!("Enter a number to get the nth number in the Fibonacci Sequence:");

    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    let mut current = first;
    for _ in 0..n {
        current = first + second;
        first = second;
        second = current;
    }

    println!("The {}th number in the Fibonacci Sequence is: {}", n, first);
}
