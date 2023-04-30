pub fn run() {
    // Go obviously have two types of "string" data structures
    // Primitive &str and the Growable string
    let mut hello = String::from("Hello World ");

    
    // You can also add to a string using the push method. However, one is a CHAR method and the other a string method
    hello.push('c'); // char push
    hello.push_str("string"); // string push

    // We could get string capacity in bytes too
    println!("Capacity {}", hello.capacity());

    // Check for empty state
    println!("Is empty {}", hello.is_empty());

    // Looping through strings

    // By whitespace
    // for word in hello.split_whitespace() {
    //     println!("{}", word);
    // }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push_str("s");

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.len());

    // Get string length
    println!("String length {}", hello.len());
    println!("{} {}", hello, s);
}