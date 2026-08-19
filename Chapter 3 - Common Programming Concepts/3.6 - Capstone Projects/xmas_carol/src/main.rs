fn main() {
    let gifts: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    let mut curr_verse: usize = 0;

    while curr_verse < 12 {
        let day = match curr_verse {
            0 => "first",
            1 => "second",
            2 => "third",
            3 => "fourth",
            4 => "fifth",
            5 => "sixth",
            6 => "seventh",
            7 => "eighth",
            8 => "ninth",
            9 => "tenth",
            10 => "eleventh",
            11 => "twelfth",
            _ => unreachable!(),
        };

        println!("\nOn the {} day of Christmas", day);
        println!("My true love gave to me");

        for i in (0..=curr_verse).rev() {
            if i == 0 && curr_verse > 0 {
                println!("And {}", gifts[i]);
            } else {
                println!("{}", gifts[i]);
            }
        }

        curr_verse += 1;
    }
}
