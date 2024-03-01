fn main() {
    let days: [&str; 12] = ["first", "second", "third", "fourth","fifth", 
        "sixth", "seventh", "eights", "ninth", "tenth", "eleventh", "twelveth"]; 
    
    let strings: [&str; 12] = ["partridge in a pear tree", "Two turtle doves", 
        "Three French hens", "Four calling birds", "Five golden rings", "Six geese a-laying", 
        "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing", 
        "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];
    
    for i in 0..days.len() {
        println!("On the {} day of Christmas,\nmy true love gave to me ", days[i]);
        
        for j in (0..i+1).rev() {
            if j != 0 {
                println!("{},", strings[j])
            }
            else {
                if i == 0 {
                    println!("A {}.", strings[j])
                } else {
                    println!("And a {}.", strings[j])
                }
            }
        }
        
        println!("")
    }
}
