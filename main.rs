
fn main() {
    
    // Create an array holding the gifts
    let gifts = [
        "A Partidge in a Pear Tree", 
        "Two Turtle Doves", 
        "Three French Hens",
        "Four Calling Brids",
        "Five Golden Rings",
        "Six Geese a Layin'",
        "Seven Swans a Swimming",
        "Eigth Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ];

    // Create a for loop for each of the 12 days of Christmas
    for day in 1..=12 {
        println!("\n0n the {} day of Christmas my true love gave to me:", ordinal(day));

        // Create a for loop that iterates through the gift array and returns the gifts
        for gift in gifts
            .iter()
            .take(day)
            .rev() {
                if day > 1 && gift == &gifts[0] {
                    print!("and ");
                }
                println!("{}", gift);
            }
    };

    // Create a function that uses ordinal to change the day number to a String 
    fn ordinal(num: usize) -> String {
        match num {
            1 => String::from("first"),
            2 => String::from("second"),
            3 => String::from("third"),
            4 => String::from("fourth"),
            5 => String::from("fifth"),
            6 => String::from("sixth"),
            7 => String::from("seventh"),
            8 => String::from("eighth"),
            9 => String::from("ninth"),
            10 => String::from("tenth"),
            11 => String::from("eleventh"),
            12 => String::from("twelfth"),
            _ => num.to_string(),
        }
    }
    
  
}
