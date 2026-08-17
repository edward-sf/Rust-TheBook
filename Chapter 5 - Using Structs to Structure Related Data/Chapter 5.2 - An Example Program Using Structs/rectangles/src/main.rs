struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let length = 30;
    let width = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_scalars(length, width)
    );
    
    let tuple = (length, width);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(tuple)
    );

    let rect1 = Rectangle {
        length: 30,
        width: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area_scalars(length: u32, width: u32) -> u32 {
    // Calculate the area of a rectangle using scalar parameters
    length * width
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    // Calculate the area of a rectangle using a tuple parameter
    dimensions.0 * dimensions.1
}

fn area(rectangle: &Rectangle) -> u32 {
    // Calculate the area of a rectangle using a struct parameter
    rectangle.length * rectangle.width
}