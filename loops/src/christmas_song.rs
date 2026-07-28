pub fn print_lyrics() {
    let numerals = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth",
        "tenth", "eleventh", "twelfth",
    ];
    for (i, num) in numerals.iter().enumerate() {
        println!("On the {} day of Christmas, my true love sent to me", num);
        print_verse(i + 1);
        println!();
    }
}

fn print_verse(n: usize) {
    let verse = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves and",
        "A partridge in a pear tree",
    ];
    let mut i: usize = 12 - n;
    while i < 12 {
        println!("{}", verse[i]);
        i += 1;
    }
}
