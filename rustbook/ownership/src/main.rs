fn main() {
    let s1 = String::from("hello world");
    // let len = calculate_length(&s1);
    // println!("The length of '{}' is {}.", s1, len);

    let word = first_word(&s1);

    // s1.clear(); // ERROR!
    // cannot borrow `s1` as mutable because it is also borrowed as immutable

    // println!(word); // ERROR!
    println!("{}", word);
}

// &: Reference
// fn calculate_length(s: &String) -> usize {
//    s.len()
// }

// &str is more general than &String:
// If we have a string slice, we can pass that directly.
// If we have a String, we can pass a slice of the entire String.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
